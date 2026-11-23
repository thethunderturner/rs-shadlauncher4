use crate::config::launcher::Launcher;
use crate::launcher::update::{self, Channel, Download, Update};
use eframe::egui;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver, Sender, TryRecvError},
};

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum Phase {
    #[default]
    Idle,
    Checking,
    Downloading,
    Installing,
}

enum Event {
    Checked(Result<Option<Update>, String>),
    Progress(f32),
    Downloaded(Result<Download, String>),
    Installed(Result<(), String>),
}

#[derive(Default)]
pub struct UpdateWindow {
    pub open: bool,
    phase: Phase,
    receiver: Option<Receiver<Event>>,
    cancel: Arc<AtomicBool>,
    update: Option<Arc<Update>>,
    progress: f32,
    error: Option<String>,
    silent: bool,
    pub channel: Channel,
}

impl UpdateWindow {
    pub fn start(&mut self, ctx: &egui::Context) {
        self.begin(ctx, false);
    }

    pub fn check_at_startup(&mut self, ctx: &egui::Context) {
        self.begin(ctx, true);
    }

    fn begin(&mut self, ctx: &egui::Context, silent: bool) {
        self.silent = silent;
        self.open = !silent;
        if self.receiver.is_some() {
            return;
        }
        self.update = None;
        let channel = self.channel;
        self.spawn(ctx, Phase::Checking, move |sender, ctx, cancel| {
            let result = update::check(channel, &cancel);
            let _ = sender.send(Event::Checked(result));
            ctx.request_repaint();
        });
        ctx.request_repaint();
    }

    fn spawn(
        &mut self,
        ctx: &egui::Context,
        phase: Phase,
        task: impl FnOnce(Sender<Event>, egui::Context, Arc<AtomicBool>) + Send + 'static,
    ) {
        let (sender, receiver) = mpsc::channel();
        self.cancel = Arc::new(AtomicBool::new(false));
        let cancel = self.cancel.clone();
        let ctx = ctx.clone();
        self.error = None;
        match std::thread::Builder::new()
            .name("launcher-update".into())
            .spawn(move || task(sender, ctx, cancel))
        {
            Ok(_) => {
                self.receiver = Some(receiver);
                self.phase = phase;
            }
            Err(error) => self.fail(format!("Could not start the update worker: {error}")),
        }
    }

    fn fail(&mut self, error: String) {
        self.receiver = None;
        self.phase = Phase::Idle;
        if self.silent {
            eprintln!("Launcher update: {error}");
        }
        self.error = Some(error);
    }

    fn poll(&mut self, ctx: &egui::Context) {
        while let Some(receiver) = &self.receiver {
            match receiver.try_recv() {
                Ok(Event::Progress(fraction)) => self.progress = fraction,
                Ok(Event::Checked(result)) => {
                    self.receiver = None;
                    self.phase = Phase::Idle;
                    match result {
                        Ok(Some(update)) => {
                            self.open = !self.silent || update.newer;
                            self.update = Some(Arc::new(update));
                        }
                        Ok(None) => {
                            self.open = !self.silent;
                        }
                        Err(error) => self.fail(error),
                    }
                    break;
                }
                Ok(Event::Downloaded(result)) => {
                    self.receiver = None;
                    match result {
                        Ok(download) => self.spawn(ctx, Phase::Installing, |sender, ctx, _| {
                            let result = update::install(download);
                            let _ = sender.send(Event::Installed(result));
                            ctx.request_repaint();
                        }),
                        Err(error) => self.fail(error),
                    }
                    break;
                }
                Ok(Event::Installed(result)) => {
                    self.receiver = None;
                    match result {
                        Ok(()) => {
                            self.open = false;
                            self.phase = Phase::Idle;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        Err(error) => self.fail(error),
                    }
                    break;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.fail("The update worker stopped unexpectedly. Try again.".into());
                    break;
                }
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, launcher: &mut Launcher) {
        self.poll(ui.ctx());
        if !self.open {
            return;
        }
        let installing = self.phase == Phase::Installing;
        if installing && ui.ctx().input(|input| input.viewport().close_requested()) {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }
        let mut open = self.open;
        let mut download = false;
        let mut retry = false;
        let mut cancel = false;
        let previous_channel = self.channel;
        let mut window = egui::Window::new("Update Launcher")
            .default_width(560.0)
            .max_width(680.0)
            .resizable(false)
            .collapsible(false);
        if !installing {
            window = window.open(&mut open);
        }
        window.show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.heading("Launcher update");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_enabled_ui(matches!(self.phase, Phase::Idle | Phase::Checking), |ui| {
                        egui::ComboBox::from_id_salt("launcher_update_channel")
                            .selected_text(self.channel.label())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.channel, Channel::Release, "Release");
                                ui.selectable_value(&mut self.channel, Channel::PreRelease, "Pre-release");
                            });
                    });
                });
            });
            ui.add_space(6.0);
            egui::Grid::new("launcher_update_versions").spacing([16.0, 6.0]).show(ui, |ui| {
                ui.label("Current commit");
                ui.monospace(update::short_commit(update::CURRENT_COMMIT))
                    .on_hover_text(update::CURRENT_COMMIT);
                ui.label(update::CURRENT_DATE.get(..10).unwrap_or("Unknown date"));
                ui.end_row();
                if let Some(update) = &self.update {
                    ui.label("Selected commit");
                    ui.hyperlink_to(update::short_commit(&update.commit), format!(
                        "https://github.com/thethunderturner/rs-shadlauncher4/commit/{}", update.commit
                    )).on_hover_text(&update.commit);
                    ui.label(update.date.get(..10).unwrap_or(&update.date));
                    ui.end_row();
                }
            });
            ui.add_space(6.0);
            if let Some(update) = &self.update {
                ui.strong(&update.version);
                ui.label(&update.message);
                ui.label(egui::RichText::new(format!("Tag: {}", update.tag)).weak());
                if !update.commits.is_empty() {
                    ui.separator();
                    ui.strong("Changes");
                    egui::ScrollArea::vertical().id_salt("launcher_update_commits")
                        .max_height(200.0).auto_shrink([false, true]).show(ui, |ui| {
                            for commit in &update.commits {
                                ui.push_id(&commit.sha, |ui| {
                                    ui.horizontal_top(|ui| {
                                        ui.hyperlink_to(update::short_commit(&commit.sha), format!(
                                            "https://github.com/thethunderturner/rs-shadlauncher4/commit/{}", commit.sha
                                        ));
                                        ui.add(egui::Label::new(commit.commit.message.lines().next().unwrap_or("No commit message")).wrap())
                                            .on_hover_text(&commit.commit.message);
                                    });
                                });
                            }
                        });
                }
                if let Some(error) = &update.changelog_error {
                    ui.add(egui::Label::new(error).wrap());
                }
                if update.available {
                    if let Some(error) = &update.install_error {
                        ui.add(egui::Label::new(error).wrap());
                    } else {
                        ui.label("The launcher will restart after updating.");
                    }
                }
            }
            if self.phase == Phase::Idle && self.update.is_none() && self.error.is_none() {
                ui.label(match self.channel {
                    Channel::Release => "No releases have been published yet. You can select Pre-release to check development builds.",
                    Channel::PreRelease => "No pre-releases have been published yet.",
                });
            }
            match self.phase {
                Phase::Checking => { ui.horizontal(|ui| { ui.spinner(); ui.label("Checking for updates…"); }); }
                Phase::Downloading => { ui.add(egui::ProgressBar::new(self.progress).show_percentage().text("Downloading update")); }
                Phase::Installing => { ui.horizontal(|ui| { ui.spinner(); ui.label("Preparing installation and restart…"); }); }
                Phase::Idle => {}
            }
            if let Some(error) = &self.error {
                ui.add(egui::Label::new(egui::RichText::new(error).color(ui.visuals().error_fg_color)).wrap());
            }
            ui.add_space(4.0);
            ui.separator();
            egui::Panel::bottom("footer").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        let can_update = self.channel == previous_channel && self.phase == Phase::Idle && self.update.as_ref()
                            .is_some_and(|update| update.available && update.install_error.is_none());
                        if self.error.is_some() {
                            retry = ui.button("Retry").clicked();
                        } else {
                            download = ui.add_enabled(can_update, egui::Button::new("Update")).clicked();
                        }
                        cancel = ui.add_enabled(!installing, egui::Button::new("Cancel")).clicked();
                    });
                });
            });
        });
        if !installing && (cancel || !open || ui.ctx().input(|input| input.key_pressed(egui::Key::Escape))) {
            self.cancel.store(true, Ordering::Relaxed);
            self.receiver = None;
            self.phase = Phase::Idle;
            self.open = false;
        } else if self.channel != previous_channel {
            // Discard the previous request so its result cannot replace this channel's result.
            self.cancel.store(true, Ordering::Relaxed);
            self.receiver = None;
            launcher.update_channel = self.channel;
            self.start(ui.ctx());
            if let Err(error) = launcher.save() {
                self.error = Some(format!("Could not save update channel: {error}"));
            }
        } else if retry {
            self.start(ui.ctx());
        } else if download {
            if let Some(update) = self.update.clone() {
                self.silent = false;
                self.progress = 0.0;
                self.spawn(ui.ctx(), Phase::Downloading, move |sender, ctx, cancel| {
                    let result = update::download(&update, &cancel, |progress| {
                        let _ = sender.send(Event::Progress(progress));
                        ctx.request_repaint();
                    });
                    let _ = sender.send(Event::Downloaded(result));
                    ctx.request_repaint();
                });
            }
        }
    }
}

impl Drop for UpdateWindow {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

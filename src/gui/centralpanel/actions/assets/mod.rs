use crate::scanning::Title;
use eframe::egui;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};

const ICON_SIZE: u32 = 80; // 40 points, with enough detail for a 2× display.
const MAX_ICONS: usize = 128;
const BACKGROUND_SIZE: (u32, u32) = (1280, 720);

#[derive(Clone, Hash, PartialEq, Eq)]
struct ImageKey {
    path: PathBuf,
    background: bool,
}

struct CachedImage {
    id: u64,
    last_used: u64,
    state: ImageState,
}

enum ImageState {
    Pending,
    Loading,
    Ready(egui::TextureHandle),
    Failed,
}

struct Request {
    key: ImageKey,
    id: u64,
    ctx: egui::Context,
}

struct LoadedImage {
    key: ImageKey,
    id: u64,
    image: Option<egui::ColorImage>,
}

// Keep small textures only. Original file bytes and decoded images are discarded
pub struct Assets {
    images: HashMap<ImageKey, CachedImage>,
    requests: Sender<Request>,
    results: Receiver<LoadedImage>,
    loading: bool,
    next_id: u64,
    frame: u64,
}

impl Default for Assets {
    fn default() -> Self {
        let (requests, jobs) = mpsc::channel::<Request>();
        let (results, loaded) = mpsc::channel();
        // One job at a time limits both decoding memory and queued pixel buffers.
        std::thread::spawn(move || {
            while let Ok(request) = jobs.recv() {
                let image = read_thumbnail(&request.key.path, request.key.background);
                if results
                    .send(LoadedImage {
                        key: request.key,
                        id: request.id,
                        image,
                    })
                    .is_err()
                {
                    break;
                }
                request.ctx.request_repaint();
            }
        });
        Self {
            images: HashMap::new(),
            requests,
            results: loaded,
            loading: false,
            next_id: 0,
            frame: 0,
        }
    }
}

impl Assets {
    pub fn begin_frame(&mut self, ctx: &egui::Context) {
        self.frame += 1;
        while let Ok(result) = self.results.try_recv() {
            self.loading = false;
            // Refresh or eviction may have removed/replaced this request.
            if let Some(entry) = self.images.get_mut(&result.key) {
                if entry.id == result.id {
                    entry.state = match result.image {
                        Some(image) => ImageState::Ready(ctx.load_texture(
                            result.key.path.to_string_lossy(),
                            image,
                            egui::TextureOptions::LINEAR,
                        )),
                        None => ImageState::Failed,
                    };
                }
            }
        }
    }

    pub fn load_icon(&mut self, title: &Title) -> Option<egui::Image<'static>> {
        let texture = self.texture(title.icon_path.as_deref()?, false)?;
        Some(
            egui::Image::new((texture.id(), texture.size_vec2()))
                .fit_to_exact_size(egui::vec2(40.0, 40.0)),
        )
    }

    pub fn show_background(&mut self, ui: &egui::Ui, title: &Title) {
        let Some(path) = title.background_path.as_deref() else {
            return;
        };
        let Some(texture) = self.texture(path, true) else {
            return;
        };
        let rect = ui.available_rect_before_wrap();
        if rect.width() <= 0.0 || rect.height() <= 0.0 {
            return;
        }

        let size = texture.size_vec2();
        let rect_aspect = rect.width() / rect.height();
        let image_aspect = size.x / size.y;
        let uv = if image_aspect > rect_aspect {
            let visible_width = rect_aspect / image_aspect;
            let left = (1.0 - visible_width) * 0.5;
            egui::Rect::from_min_max(
                egui::pos2(left, 0.0),
                egui::pos2(left + visible_width, 1.0),
            )
        } else {
            let visible_height = image_aspect / rect_aspect;
            let top = (1.0 - visible_height) * 0.5;
            egui::Rect::from_min_max(
                egui::pos2(0.0, top),
                egui::pos2(1.0, top + visible_height),
            )
        };
        ui.painter().image(
            texture.id(),
            rect,
            uv,
            egui::Color32::WHITE,
        );
        ui.painter()
            .rect_filled(rect, 0.0, ui.visuals().panel_fill.gamma_multiply(0.85));
    }

    fn texture(&mut self, path: &Path, background: bool) -> Option<&egui::TextureHandle> {
        let key = ImageKey {
            path: path.to_owned(),
            background,
        };
        let entry = self.images.entry(key).or_insert_with(|| {
            self.next_id += 1;
            CachedImage {
                id: self.next_id,
                last_used: self.frame,
                state: ImageState::Pending,
            }
        });
        entry.last_used = self.frame;
        match &entry.state {
            ImageState::Ready(texture) => Some(texture),
            _ => None,
        }
    }

    pub fn end_frame(&mut self, ctx: &egui::Context) {
        // Keep only the displayed background. Do not free textures painted this frame.
        self.images
            .retain(|key, entry| !key.background || entry.last_used == self.frame);
        let icon_count = self.images.keys().filter(|key| !key.background).count();
        if icon_count > MAX_ICONS {
            let mut unused: Vec<_> = self
                .images
                .iter()
                .filter(|(key, entry)| !key.background && entry.last_used != self.frame)
                .map(|(key, entry)| (key.clone(), entry.last_used))
                .collect();
            unused.sort_unstable_by_key(|(_, frame)| *frame);
            for (key, _) in unused.into_iter().take(icon_count - MAX_ICONS) {
                self.images.remove(&key);
            }
        }

        if !self.loading {
            // Only load images currently visible, giving the selected background priority.
            let next = self
                .images
                .iter_mut()
                .filter(|(_, entry)| {
                    entry.last_used == self.frame && matches!(entry.state, ImageState::Pending)
                })
                .min_by_key(|(key, entry)| (!key.background, entry.id));
            if let Some((key, entry)) = next {
                let request = Request {
                    key: key.clone(),
                    id: entry.id,
                    ctx: ctx.clone(),
                };
                if self.requests.send(request).is_ok() {
                    entry.state = ImageState::Loading;
                    self.loading = true;
                } else {
                    entry.state = ImageState::Failed;
                }
            }
        }
    }

    // Refresh reloads files from disk. An old in-flight result will be ignored
    pub fn clear_cache(&mut self) {
        self.images.clear();
    }
}

fn read_thumbnail(path: &Path, background: bool) -> Option<egui::ColorImage> {
    let size = if background {
        BACKGROUND_SIZE
    } else {
        (ICON_SIZE, ICON_SIZE)
    };
    let original = image::ImageReader::open(path).ok()?.decode().ok()?;
    let thumbnail = if original.width() > size.0 || original.height() > size.1 {
        let thumbnail = original.thumbnail(size.0, size.1);
        drop(original);
        thumbnail
    } else {
        original
    }
    .into_rgba8();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [thumbnail.width() as usize, thumbnail.height() as usize],
        thumbnail.as_raw(),
    ))
}

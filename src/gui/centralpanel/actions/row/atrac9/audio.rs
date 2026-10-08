use crate::gui::centralpanel::actions::row::atrac9::at9::At9;
use crate::scanning::Title;
use rodio::{OutputStreamBuilder, Sink, buffer::SamplesBuffer};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::Duration;

#[derive(Default)]
pub struct Audio {
    requests: Option<Sender<Option<Vec<PathBuf>>>>,
}

impl Audio {
    pub fn play(&mut self, title: &Title) {
        // Resolve files on the worker, as the library may live on a slow drive.
        let paths = title
            .patch
            .iter()
            .map(|patch| &patch.path)
            .chain(std::iter::once(&title.app.path))
            .map(|path| path.join("sce_sys/snd0.at9"))
            .collect();
        let requests = self.requests.get_or_insert_with(|| {
            let (tx, rx) = mpsc::channel();
            std::thread::spawn(move || playback_worker(rx));
            tx
        });
        let _ = requests.send(Some(paths));
    }

    pub fn stop(&self) {
        if let Some(requests) = &self.requests {
            let _ = requests.send(None);
        }
    }
}

// This thread owns the output device, decoder, and sink. Dropping Audio closes
// the channel and releases them; neither decoding nor device setup blocks egui.
fn playback_worker(requests: Receiver<Option<Vec<PathBuf>>>) {
    let mut stream = None;
    let mut sink: Option<Sink> = None;
    let mut audio: Option<At9> = None;
    loop {
        let request = if sink.is_some() {
            requests.recv_timeout(Duration::from_millis(10))
        } else {
            requests.recv().map_err(|_| RecvTimeoutError::Disconnected)
        };
        match request {
            Ok(mut paths) => {
                // Rapid clicks replace pending requests instead of queuing tracks.
                for newest in requests.try_iter() {
                    paths = newest;
                }
                if let Some(previous) = sink.take() {
                    previous.stop();
                }
                audio = None;
                let Some(path) =
                    paths.and_then(|paths| paths.into_iter().find(|path| path.is_file()))
                else {
                    stream = None;
                    continue;
                };
                let decoder = match At9::open(&path) {
                    Ok(decoder) => decoder,
                    Err(error) => {
                        eprintln!("Could not play {}: {error}", path.display());
                        stream = None;
                        continue;
                    }
                };
                if stream.is_none() {
                    match OutputStreamBuilder::open_default_stream() {
                        Ok(mut output) => {
                            output.log_on_drop(false);
                            stream = Some(output);
                        }
                        Err(error) => {
                            eprintln!("Could not open audio output: {error}");
                            continue;
                        }
                    }
                }
                if let Some(output) = &stream {
                    sink = Some(Sink::connect_new(output.mixer()));
                    audio = Some(decoder);
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
        if let Some(player) = &sink {
            // At most four superframes queued: no full-song PCM allocation/cache.
            for _ in 0..4 {
                if player.len() >= 4 {
                    break;
                }
                let Some(decoder) = audio.as_mut() else { break };
                match decoder.next_buffer() {
                    Ok(Some(samples)) => player.append(SamplesBuffer::new(
                        decoder.channels,
                        decoder.sample_rate,
                        samples,
                    )),
                    Ok(None) => audio = None,
                    Err(error) => {
                        eprintln!("Could not decode snd0.at9: {error}");
                        player.stop();
                        audio = None;
                    }
                }
            }
            if audio.is_none() && player.empty() {
                sink = None;
                stream = None;
            }
        }
    }
}

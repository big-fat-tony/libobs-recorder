pub use adapter::{Adapter, AdapterId, AdapterType};
pub use audio::AudioSource;
pub use cover::Cover;
pub use encoders::Encoder;
pub use framerate::Framerate;
pub use overlay::{Corner, TextOverlay};
pub use rate_control::RateControl;
pub use resolution::{Resolution, StdResolution};
pub use window::Window;

mod adapter;
mod audio;
mod cover;
mod encoders;
mod framerate;
mod overlay;
mod rate_control;
mod resolution;
mod window;

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct RecorderSettings {
    /// ID of GPU
    pub(crate) window: Window,
    pub(crate) input_resolution: Resolution,
    pub(crate) output_resolution: Resolution,
    pub(crate) output_path: String,
    pub(crate) framerate: Option<Framerate>,
    pub(crate) rate_control: Option<RateControl>,
    pub(crate) audio_source: Option<AudioSource>,
    pub(crate) encoder: Option<Encoder>,
    #[serde(default)]
    pub(crate) text_overlay: Option<TextOverlay>,
    #[serde(default)]
    pub(crate) audio_tracks: Option<Vec<AudioSource>>,
    #[serde(default)]
    pub(crate) microphone_gain_db: Option<f32>,
    #[serde(default)]
    pub(crate) desktop_gain_db: Option<f32>,
    #[serde(default)]
    pub(crate) cover: Option<Cover>,
}

impl RecorderSettings {
    pub fn new(
        window: Window,
        input_resolution: impl Into<Resolution>,
        output_resolution: impl Into<Resolution>,
        output_path: impl AsRef<std::path::Path>,
    ) -> Self {
        let input_resolution = input_resolution.into();
        let output_resolution = output_resolution.into();
        let output_path = output_path
            .as_ref()
            .to_str()
            .expect("expected unicode path")
            .to_string();

        Self {
            window,
            input_resolution,
            output_resolution,
            output_path,
            framerate: None,
            rate_control: None,
            audio_source: None,
            encoder: None,
            text_overlay: None,
            audio_tracks: None,
            microphone_gain_db: None,
            desktop_gain_db: None,
            cover: None,
        }
    }

    pub fn set_window(&mut self, window: Window) {
        self.window = window;
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn set_input_resolution(&mut self, size: impl Into<Resolution>) {
        self.input_resolution = size.into();
    }

    pub fn get_input_resolution(&self) -> &Resolution {
        &self.input_resolution
    }

    pub fn set_output_resolution(&mut self, resolution: impl Into<Resolution>) {
        self.output_resolution = resolution.into();
    }

    pub fn get_output_resolution(&self) -> &Resolution {
        &self.output_resolution
    }

    pub fn set_output_path(&mut self, output_path: impl Into<String>) {
        self.output_path = output_path.into();
    }

    pub fn get_output_path(&self) -> &str {
        self.output_path.as_str()
    }

    pub fn set_framerate(&mut self, framerate: Framerate) {
        self.framerate = Some(framerate);
    }

    pub fn get_framerate(&self) -> Option<&Framerate> {
        self.framerate.as_ref()
    }

    pub fn set_rate_control(&mut self, rate_control: RateControl) {
        self.rate_control = Some(rate_control);
    }

    pub fn get_rate_control(&self) -> Option<&RateControl> {
        self.rate_control.as_ref()
    }

    pub fn set_audio_source(&mut self, record_audio: AudioSource) {
        self.audio_source = Some(record_audio);
    }

    pub fn get_audio_source(&self) -> Option<&AudioSource> {
        self.audio_source.as_ref()
    }

    pub fn set_encoder(&mut self, encoder: Encoder) {
        self.encoder = Some(encoder);
    }

    pub fn get_encoder(&self) -> Option<&Encoder> {
        self.encoder.as_ref()
    }

    /// Burn a text overlay into the recording. `None` (the default) records
    /// the bare window.
    pub fn set_text_overlay(&mut self, overlay: TextOverlay) {
        self.text_overlay = Some(overlay);
    }

    pub fn clear_text_overlay(&mut self) {
        self.text_overlay = None;
    }

    pub fn get_text_overlay(&self) -> Option<&TextOverlay> {
        self.text_overlay.as_ref()
    }

    /// Record one audio stream per entry instead of a single mixed one — e.g.
    /// `[AudioSource::APPLICATION, AudioSource::ALL]` writes the captured
    /// window's audio as track 1 and everything plus the microphone as track 2.
    /// Takes precedence over [`Self::set_audio_source`]; entries beyond
    /// [`MAX_AUDIO_TRACKS`] are ignored.
    pub fn set_audio_tracks(&mut self, tracks: Vec<AudioSource>) {
        self.audio_tracks = Some(tracks);
    }

    pub fn clear_audio_tracks(&mut self) {
        self.audio_tracks = None;
    }

    pub fn get_audio_tracks(&self) -> Option<&[AudioSource]> {
        self.audio_tracks.as_deref()
    }

    /// Amplify (or attenuate) the microphone by `db` before it is mixed into
    /// its track, for when a headset sits far below the voices coming out of
    /// the desktop. `None` (the default) records it as the device delivers it.
    pub fn set_microphone_gain_db(&mut self, db: f32) {
        self.microphone_gain_db = Some(db);
    }

    pub fn get_microphone_gain_db(&self) -> Option<f32> {
        self.microphone_gain_db
    }

    /// Attenuate (or amplify) the desktop capture by `db` before it is mixed.
    /// A negative value buys the headroom a track needs when it carries the
    /// microphone as well: Windows hands out a desktop mix that already sits
    /// at full scale, and a sum past full scale is clamped, which crackles.
    pub fn set_desktop_gain_db(&mut self, db: f32) {
        self.desktop_gain_db = Some(db);
    }

    pub fn get_desktop_gain_db(&self) -> Option<f32> {
        self.desktop_gain_db
    }

    /// Paint a filled rectangle over part of the capture — see [`Cover`].
    /// `None` (the default) records the window as it is.
    pub fn set_cover(&mut self, cover: Cover) {
        self.cover = Some(cover);
    }

    pub fn clear_cover(&mut self) {
        self.cover = None;
    }

    pub fn get_cover(&self) -> Option<&Cover> {
        self.cover.as_ref()
    }
}

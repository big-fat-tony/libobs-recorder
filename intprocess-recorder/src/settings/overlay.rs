/// Which corner of the output a text overlay is anchored to.
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// A line of text burned into a corner of the recording (a clock, a label…).
/// The text can be replaced while recording with `Recorder::set_overlay_text`.
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct TextOverlay {
    pub(crate) text: String,
    /// Font size in pixels of the *output* canvas.
    pub(crate) font_size: u32,
    pub(crate) corner: Corner,
    /// Distance from the two nearest edges, in output pixels.
    pub(crate) margin: u32,
    pub(crate) font_face: String,
}

impl TextOverlay {
    pub fn new(text: impl Into<String>, font_size: u32, corner: Corner) -> Self {
        Self {
            text: text.into(),
            font_size,
            corner,
            margin: 16,
            font_face: "Arial".into(),
        }
    }

    pub fn set_margin(&mut self, margin: u32) {
        self.margin = margin;
    }

    pub fn set_font_face(&mut self, face: impl Into<String>) {
        self.font_face = face.into();
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_font_size(&self) -> u32 {
        self.font_size
    }

    pub fn get_corner(&self) -> Corner {
        self.corner
    }
}

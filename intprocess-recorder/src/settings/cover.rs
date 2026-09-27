/// A filled rectangle painted over part of the capture, for hiding something
/// the recording should not carry — a chat window with other people's messages
/// in it, an addon showing a name, a second monitor's notification.
///
/// The rectangle is given in fractions of the frame rather than pixels, so the
/// same setting survives a change of resolution or a downscaled output.
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Cover {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    /// Fill colour as `0xAABBGGRR`, libobs' order. Opaque black by default.
    pub(crate) color: u32,
}

impl Cover {
    /// `x`/`y` are the top-left corner and `width`/`height` the size, each as a
    /// fraction of the frame in `0.0..=1.0`. Anything outside that is clamped,
    /// and a rectangle reaching past an edge is trimmed to it.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let x = x.clamp(0.0, 1.0);
        let y = y.clamp(0.0, 1.0);
        Self {
            x,
            y,
            width: width.clamp(0.0, 1.0 - x),
            height: height.clamp(0.0, 1.0 - y),
            color: 0xFF00_0000,
        }
    }

    /// Fill colour as `0xAARRGGBB`, the order a person writes it in.
    pub fn set_color_argb(&mut self, argb: u32) {
        let (a, r, g, b) = (argb >> 24 & 0xFF, argb >> 16 & 0xFF, argb >> 8 & 0xFF, argb & 0xFF);
        self.color = a << 24 | b << 16 | g << 8 | r;
    }

    /// The rectangle in pixels of a frame this size, as (x, y, width, height).
    pub(crate) fn in_pixels(&self, width: u32, height: u32) -> (f32, f32, u32, u32) {
        let (w, h) = (width as f32, height as f32);
        (
            self.x * w,
            self.y * h,
            (self.width * w).round().max(1.0) as u32,
            (self.height * h).round().max(1.0) as u32,
        )
    }

    /// Is there anything to paint?
    pub(crate) fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::Cover;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn a_rectangle_past_the_edge_is_trimmed_to_it() {
        let cover = Cover::new(0.9, 0.9, 0.5, 0.5);
        assert!(close(cover.width, 0.1) && close(cover.height, 0.1), "got {cover:?}");
    }

    #[test]
    fn fractions_become_pixels_of_whatever_frame_is_being_recorded() {
        let (x, y, w, h) = Cover::new(0.01, 0.75, 0.21, 0.21).in_pixels(2560, 1440);
        assert!(close(x, 25.6) && close(y, 1080.0), "got {x}, {y}");
        assert_eq!((w, h), (538, 302));
        // Same setting, half the canvas: the same part of the picture.
        let (x, y, w, h) = Cover::new(0.01, 0.75, 0.21, 0.21).in_pixels(1280, 720);
        assert!(close(x, 12.8) && close(y, 540.0), "got {x}, {y}");
        assert_eq!((w, h), (269, 151));
    }

    #[test]
    fn colors_are_given_in_argb_and_stored_the_way_libobs_wants_them() {
        let mut cover = Cover::new(0.0, 0.0, 1.0, 1.0);
        assert_eq!(cover.color, 0xFF00_0000, "opaque black by default");
        cover.set_color_argb(0x8012_3456);
        assert_eq!(cover.color, 0x8056_3412);
    }
}

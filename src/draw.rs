use Matrix;

/// Trait for a context that can handle drawing.
pub trait Draw {
    /// Type of a resource that represents an image.
    type ImageResource: ?Sized;

    /// Type of a resource that represents a font.
    type FontResource: ?Sized;

    /// Draws an image that covers the whole surface (from `-1.0` to `1.0` both horizontally and
    /// vertically), but multiplied by the matrix.
    ///
    /// This function should not try to preseve the aspect ratio of the image. This is handled by
    /// the rest of the library.
    fn draw_image(&mut self, name: &Self::ImageResource, matrix: &Matrix);

    /// Given an image, this functions returns its width divided by its height.
    fn get_image_width_per_height(&mut self, name: &Self::ImageResource) -> f32;

    /// Draws an image that covers the whole surface (from `-1.0` to `1.0` both horizontally and
    /// vertically), but multiplied by the matrix.
    ///
    /// This function should not try to preseve the aspect ratio of the text. This is handled by
    /// the rest of the library.
    fn draw_text(&mut self, font: &Self::FontResource, matrix: &Matrix, text: &str);

    /// Given a font and a text, this function returns the width the text would have on the screen
    /// divided by a EM.
    fn get_text_width_per_em(&mut self, font: &Self::FontResource, text: &str) -> f32;
}

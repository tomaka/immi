use Matrix;

/// Trait for a context that can handle drawing.
pub trait Draw {
    type ImageResource: ?Sized;
    type FontResource: ?Sized;

    fn draw_image(&mut self, name: &Self::ImageResource, matrix: &Matrix);

    fn get_image_width_per_height(&mut self, name: &Self::ImageResource) -> f32;

    fn draw_text(&mut self, font: &Self::FontResource, matrix: &Matrix, text: &str);

    fn get_text_width_per_em(&mut self, font: &Self::FontResource, text: &str) -> f32;
}

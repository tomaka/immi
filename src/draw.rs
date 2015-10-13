use Matrix;

/// Trait for a context that can handle drawing.
pub trait Draw {
    type Resource: ?Sized;

    fn draw_image(&mut self, name: &Self::Resource, matrix: &Matrix);

    fn get_image_width_per_height(&mut self, name: &Self::Resource) -> f32;

    fn draw_text(&mut self, font: &Self::Resource, matrix: &Matrix, text: &str);

    fn get_text_width_per_em(&mut self, font: &Self::Resource, text: &str) -> f32;
}

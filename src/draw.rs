use Matrix;

/// Trait for a context that can handle drawing.
pub trait Draw {
    /// Type of a resource that represents an image.
    type ImageResource: ?Sized;

    /// Type of a resource that represents a font.
    type FontResource: ?Sized;

    /// Draws a single triangle that covers the top-left hand corner of the surface, multiplied
    /// by the matrix. In other words, the OpenGL positions of the triangle are `[-1.0, 1.0]`,
    /// `[-1.0, -1.0]` and `[1.0, 1.0]`.
    ///
    /// The UV coordinates correspond to the UV coordinates of the texture. `[0.0, 0.0]` is the
    /// bottom-left hand corner of the texture, and `[1.0, 1.0]` is the top-right hand corner.
    fn draw_triangle(&mut self, texture: &Self::ImageResource, matrix: &Matrix,
                     uv_coords: [[f32; 2]; 3]);

    /// Draws an image that covers the whole surface (from `-1.0` to `1.0` both horizontally and
    /// vertically), but multiplied by the matrix.
    ///
    /// This function should not try to preseve the aspect ratio of the image. This is handled by
    /// the rest of the library.
    fn draw_image(&mut self, name: &Self::ImageResource, matrix: &Matrix) {
        self.draw_triangle(name, matrix, [[0.0, 1.0], [0.0, 0.0], [1.0, 1.0]]);

        let invert = Matrix::scale(-1.0);
        self.draw_triangle(name, &(*matrix * invert), [[1.0, 0.0], [1.0, 1.0], [0.0, 0.0]]);
    }

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

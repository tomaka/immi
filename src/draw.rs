// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use Matrix;

/// Trait for a context that can handle drawing.
pub trait DrawImage<I: ?Sized> {
    /// Draws a single triangle that covers the top-left hand corner of the surface, pre-multiplied
    /// by the matrix.
    ///
    /// To do so, draw a triangle whose coordinatges are `[-1.0, 1.0]`, `[-1.0, -1.0]`
    /// and `[1.0, 1.0]`, then pre-muliplty these coordinates with the matrix given as parameter.
    /// If you use Vulkan or DirectX, you have to perform an additional step. Pre-multiply that
    /// result with a matrix that inverts the `y` coordinate (ie. an identity matrix but whose
    /// value at the second row of the second column is `-1.0`).
    ///
    /// The UV coordinates passed as parameter are respectively the texture coordinates at the
    /// top-left, bottom-left and top-right corners. `[0.0, 0.0]` is the bottom-left hand corner
    /// of the texture, and `[1.0, 1.0]` is the top-right hand corner. If you use OpenGL, you can
    /// pass through the values. If you use DirectX or Vulkan, you must do `y = 1.0 - y` somewhere.
    fn draw_triangle(&mut self, texture: &I, matrix: &Matrix,
                     uv_coords: [[f32; 2]; 3]);

    /// Draws an image that covers the whole surface (from `-1.0` to `1.0` both horizontally and
    /// vertically), but multiplied by the matrix.
    ///
    /// This function should not try to preseve the aspect ratio of the image. This is handled by
    /// the caller.
    #[inline]
    fn draw_image(&mut self, name: &I, matrix: &Matrix) {
        self.draw_image_uv(name, matrix, [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0])
    }

    /// Draws an image that covers the whole surface (from `-1.0` to `1.0` both horizontally and
    /// vertically), but multiplied by the matrix.
    ///
    /// This function should not try to preseve the aspect ratio of the image. This is handled by
    /// the caller.
    ///
    /// Contrary to `draw_image`, this library allows one to specify UV coordinates of the four
    /// borders. Coordinates `[0.0, 0.0]` correspond to the bottom-left hand corner of the
    /// image, and `[1.0, 1.0]` correspond to the top-right hand corner.
    #[inline]
    fn draw_image_uv(&mut self, name: &I, matrix: &Matrix, top_left: [f32; 2],
                     top_right: [f32; 2], bottom_right: [f32; 2], bottom_left: [f32; 2])
    {
        self.draw_triangle(name, matrix, [top_left, bottom_left, top_right]);

        let invert = Matrix::scale(-1.0);
        self.draw_triangle(name, &(*matrix * invert), [bottom_right, top_right, bottom_left]);
    }

    /// Given an image, this functions returns its width divided by its height.
    fn get_image_width_per_height(&mut self, name: &I) -> f32;
}

pub trait DrawText<T: ?Sized> {
    /// Does the same as `draw_image`, but draws a glyph of a text instead.
    fn draw_glyph(&mut self, text_style: &T, glyph: char, matrix: &Matrix);

    /// Returns the height of a line of text in EMs.
    ///
    /// This value is usually somewhere around `1.2`.
    fn line_height(&self, text_style: &T) -> f32;

    /// Returns information about a specific glyph.
    fn glyph_infos(&self, text_style: &T, glyph: char) -> GlyphInfos;

    /// Returns the kerning between two characters for the given font.
    ///
    /// The kerning is an offset to add to the position of a specific character when it follows
    /// another specific character. For example when you write `To`, thanks to kerning the `o`
    /// can slip under the `T`, which looks nicer than if they were simply next to each other.
    ///
    /// A positive value moves the second character further away from the first one, while a
    /// negative values moves the second character next to the first one. The value must be a
    /// multiple of 1 em. When in doubt, you can simply return `0.0`.
    fn kerning(&self, text_style: &T, first_char: char, second_char: char) -> f32;
}

/// Information about a single glyph.
///
/// All the values of this struct must be relative to the size of an EM, so that the library can
/// adjust the values to any size.
#[derive(Debug, Copy, Clone)]
pub struct GlyphInfos {
    /// Width of the glyph in pixels, divided by the number of pixels of an EM.
    pub width: f32,

    /// Height of the glyph in pixels, divided by the number of pixels of an EM.
    ///
    /// By definition, this value is supposed to be always 1.0 for the glyph 'M'. In practice this
    /// is not always exactly true.
    pub height: f32,

    /// Number of pixels from the end of the previous glyph to the start of this one, divided by
    /// the number of pixels of an EM.
    pub x_offset: f32,

    /// Number of pixels from the base of the line to the top of this one, divided by the number
    /// of pixels of an EM.
    ///
    /// For glyphs that don't go under the line (like 'm' or 'u' for example), this is equal to
    /// the height of the glyph. For glyphs that go under the line (like 'p' or 'g'), this is
    /// equal to the height of the glyph minus the portion that goes under the line.
    pub y_offset: f32,

    /// Number of pixels from the end of the previous glyph to the end of this one, divided by
    /// the number of pixels of an EM.
    ///
    /// Should always be superior to `width + x_offset`.
    pub x_advance: f32,
}

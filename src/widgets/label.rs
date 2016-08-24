use std::mem;

use Alignment;
use Draw;
use DrawContext;
use HorizontalAlignment;
use matrix::Matrix;

pub fn contain<D: ?Sized + Draw>(draw: &DrawContext<D>, text_style: &D::TextStyle, text: &str,
                                 alignment: &Alignment)
{
    helper(draw, text_style, text, |ratio| {
        let draw = draw.enforce_aspect_ratio_downscale(ratio, alignment);

        if !draw.cursor_hovered_widget() {
            if draw.is_cursor_hovering() {
                draw.set_cursor_hovered_widget();
            }
        }

        draw.matrix().clone()
    })
}

pub fn cover<D: ?Sized + Draw>(draw: &DrawContext<D>, text_style: &D::TextStyle, text: &str,
                               alignment: &Alignment)
{
    helper(draw, text_style, text, |ratio| {
        let draw = draw.enforce_aspect_ratio_upscale(ratio, alignment);

        if !draw.cursor_hovered_widget() {
            if draw.is_cursor_hovering() {
                draw.set_cursor_hovered_widget();
            }
        }

        draw.matrix().clone()
    })
}

/// The text will use the current height and will stretch horizontally as needed to preserve the
/// correct aspect ratio.
pub fn flow<D: ?Sized + Draw>(draw: &DrawContext<D>, text_style: &D::TextStyle, text: &str,
                              alignment: &HorizontalAlignment)
{
    helper(draw, text_style, text, |ratio| {
        let current_width_per_height = draw.width_per_height();
        let draw = draw.horizontal_rescale(ratio / current_width_per_height, &alignment);

        if !draw.cursor_hovered_widget() {
            if draw.is_cursor_hovering() {
                draw.set_cursor_hovered_widget();
            }
        }

        draw.matrix().clone()
    })
}



fn helper<D: ?Sized + Draw, F>(draw: &DrawContext<D>, text_style: &D::TextStyle, text: &str,
                               final_matrix: F)
    where F: FnOnce(f32) -> Matrix
{
    let mut glyphs: Vec<(char, Matrix)> = Vec::with_capacity(text.len());

    let mut previous_chr = None;
    let mut x = 0.0;
    for chr in text.chars() {
        let glyph_infos = draw.draw().glyph_infos(text_style, chr);
        let kerning = match mem::replace(&mut previous_chr, Some(chr)) {
            Some(prev) => draw.draw().kerning(text_style, prev, chr),
            None => 0.0
        };

        x += kerning;

        let matrix = Matrix::translate(x + glyph_infos.x_offset,
                                       glyph_infos.y_offset - glyph_infos.height)
            * Matrix::scale_wh(glyph_infos.width, glyph_infos.height)
            * Matrix::translate(0.5, 0.5)
            * Matrix::scale(0.5);

        glyphs.push((chr, matrix));
        x += glyph_infos.x_advance;
    }

    // `x` now contains the width of the text in ems.

    // In the code above, we moved each character so that (0, 0) is the bottom-left corner of
    // the first character. We have to move everything so that it becomes (-1, -1) again.
    let recenter_matrix = Matrix::scale_wh(2.0 / x, 2.0)
            * Matrix::translate(-x / 2.0, -0.75);       // TODO: why -0.75? don't know. In theory -0.5 should be the value, but -0.75 is the one that works in practice

    let final_matrix = final_matrix(x);

    for (chr, matrix) in glyphs.into_iter() {
        draw.draw().draw_glyph(text_style, chr, &(final_matrix * recenter_matrix * matrix));
    } 
}

// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A label is a single line of text.
//!
//! You usually want to use the `flow` function.

use std::mem;

use Alignment;
use DrawText;
use DrawContext;
use HorizontalAlignment;
use matrix::Matrix;

/// Draws text. The text will always have the same height as the context and will stretch
/// horizontally as needed to have a correct aspect ratio.
///
/// This is usually the function that you want in order to draw text. Even though the text
/// can overflow its container if it is too long, it is usually visually better to have an
/// overflow than to have multiple texts of different heights when they should be the same.
pub fn flow<D: ?Sized + DrawText<T>, T: ?Sized>(draw: &DrawContext<D>, text_style: &T, text: &str,
                                                alignment: &HorizontalAlignment)
{
    let draw = draw.animation_stop();
    helper(&draw, text_style, text, |ratio| {
        let current_width_per_height = draw.width_per_height();
        let draw = draw.horizontal_rescale(ratio / current_width_per_height, &alignment);

        if !draw.cursor_hovered_widget() {
            if draw.is_cursor_hovering() {
                draw.set_cursor_hovered_widget();
            }
        }

        draw.matrix()
    })
}

/// Draws text. The text will be sized so that it is entirely contained within the context, and
/// either its width or its height is equal to the width or the height of the context.
pub fn contain<D: ?Sized + DrawText<T>, T: ?Sized>(draw: &DrawContext<D>, text_style: &T, text: &str,
                                                   alignment: &Alignment)
{
    let draw = draw.animation_stop();
    helper(&draw, text_style, text, |ratio| {
        let draw = draw.enforce_aspect_ratio_downscale(ratio, alignment);

        if !draw.cursor_hovered_widget() {
            if draw.is_cursor_hovering() {
                draw.set_cursor_hovered_widget();
            }
        }

        draw.matrix()
    })
}

/// Draws text. The text will be sized so that it entirely covers the context, and either its
/// width or its height is equal to the width or the height of the context.
pub fn cover<D: ?Sized + DrawText<T>, T: ?Sized>(draw: &DrawContext<D>, text_style: &T, text: &str,
                                                 alignment: &Alignment)
{
    let draw = draw.animation_stop();
    helper(&draw, text_style, text, |ratio| {
        let draw = draw.enforce_aspect_ratio_upscale(ratio, alignment);

        if !draw.cursor_hovered_widget() {
            if draw.is_cursor_hovering() {
                draw.set_cursor_hovered_widget();
            }
        }

        draw.matrix()
    })
}

fn helper<D: ?Sized + DrawText<T>, T: ?Sized, F>(draw: &DrawContext<D>, text_style: &T, text: &str,
                                                 final_matrix: F)
    where F: FnOnce(f32) -> Matrix
{
    let mut glyphs: Vec<(char, Matrix)> = Vec::with_capacity(text.len());

    let mut previous_chr = None;
    let mut x = 0.0;
    for chr in text.chars() {
        let glyph_infos = draw.draw().glyph_infos(text_style, chr);
        let kerning = match mem::replace(&mut previous_chr, Some((chr, glyph_infos))) {
            Some((prev, _)) => draw.draw().kerning(text_style, prev, chr),
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

    if let Some((_, prev_infos)) = previous_chr {
        x -= prev_infos.x_advance;
        x += prev_infos.x_offset;
        x += prev_infos.width;
    }

    // `x` now contains the width of the text in ems.

    // So far the matrix of each character is in a coordinate system where 1.0 unit is equal to 1.0
    // EM and the bottom-left corner of the first glyph is 0.0. Y=1.0 is the top of the line of
    // text. We have to adjust this coordinates system for the final output.
    let recenter_matrix = Matrix::scale_wh(2.0 / x, 2.0)
            * Matrix::translate(-x / 2.0, -0.5);

    let final_matrix = final_matrix(x);

    for (chr, matrix) in glyphs.into_iter() {
        draw.draw().draw_glyph(text_style, chr, &(final_matrix * recenter_matrix * matrix));
    } 
}

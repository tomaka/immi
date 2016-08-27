// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Non-interactive images used for decoration purposes.
//!
//! If you want to use an image as a button, see the other modules.

use Alignment;
use Draw;
use DrawContext;

/// Decreases the size of the image if necessary until it fits in the context, then draws it.
pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, image_name: &D::ImageResource,
                              alignment: &Alignment)
{
    let draw = draw.animation_stop();

    let ratio = draw.draw().get_image_width_per_height(image_name);
    stretch(&draw.enforce_aspect_ratio_downscale(ratio, alignment), image_name)
}

/// Stretches the image if necessary so that it corresponds to the context's area, then draws it.
pub fn stretch<D: ?Sized + Draw>(draw: &DrawContext<D>, image_name: &D::ImageResource) {
    if !draw.cursor_hovered_widget() {
        if draw.is_cursor_hovering() {
            draw.set_cursor_hovered_widget();
        }
    }

    draw.draw().draw_image(image_name, &draw.matrix());
}

/// Increases the size of the image until it covers the context, then draws it.
pub fn cover<D: ?Sized + Draw>(draw: &DrawContext<D>, image_name: &D::ImageResource,
                              alignment: &Alignment)
{
    let draw = draw.animation_stop();
    let ratio = draw.draw().get_image_width_per_height(image_name);
    stretch(&draw.enforce_aspect_ratio_upscale(ratio, alignment), image_name)
}

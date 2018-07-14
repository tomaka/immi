// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A progress bar is a rectangle that fills itself to indicate some sort of progression.
//! 
//! A widget like this is composed of two images:
//! 
//! - The widget when empty.
//! - The widget when full. Since this one is drawn over the previous one, it can also just be the
//!   difference between empty and full.
//!

use Alignment;
use DrawImage;
use DrawContext;
use HorizontalAlignment;

use widgets::image;

/// Draws a progress bar and keeps the aspect ratio of the empty image.
///
/// If the `full` image doesn't have the same aspect ratio, it will be stretched.
///
/// # Panic
///
/// Panicks if `progress` is not between 0.0 and 1.0.
#[inline]
pub fn draw<D: ?Sized + DrawImage<I>, I: ?Sized>(draw: &DrawContext<D>, empty: &I,
                                                 full: &I, progress: f32,
                                                 progress_direction: &HorizontalAlignment,
                                                 alignment: &Alignment)
{
    let draw = draw.animation_stop();
    let ratio = draw.draw().get_image_width_per_height(empty);
    stretch(&draw.enforce_aspect_ratio_downscale(ratio, alignment), empty, full, progress,
            progress_direction)
}

/// Draws a progress bar, stretching it over the whole area.
///
/// # Panic
///
/// Panicks if `progress` is not between 0.0 and 1.0.
pub fn stretch<D: ?Sized + DrawImage<I>, I: ?Sized>(draw: &DrawContext<D>, empty: &I,
                                                    full: &I, progress: f32,
                                                    progress_direction: &HorizontalAlignment)
{
    assert!(progress >= 0.0);
    assert!(progress <= 1.0);

    // Drawing the empty image.
    image::stretch(draw, empty);

    // Drawing the full image.
    let draw = draw.horizontal_rescale(progress, progress_direction);
    draw.draw().draw_image_uv(full, &draw.matrix(), [0.0, 1.0], [progress, 1.0], [progress, 0.0],
                              [0.0, 0.0]);
}

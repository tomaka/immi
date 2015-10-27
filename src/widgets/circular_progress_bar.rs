//! A circular progress bar is a circle that fills itself to indicate some sort of progression.
//! 
//! A widget like this is composed of two images:
//! 
//! - The widget when empty.
//! - The widget when full. Since this one is drawn over the previous one, it can also just be the
//!   difference between empty and full.
//! 
//! This module supposes that the center of the circular progress bar is the center of the image.
//! The direction is always clockwise. <-- TODO: allow choosing this
//!
use Alignment;
use Draw;
use DrawContext;
use Matrix;

use widgets::image;

/// Draws a circular progress bar and keeps the aspect ratio of the empty image.
///
/// If the `full` image doesn't have the same aspect ratio, it will be stretched.
///
/// # Panic
///
/// Panicks if `progress` is not between 0.0 and 1.0.
#[inline]
pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, empty: &D::ImageResource,
                              full: &D::ImageResource, progress: f32, alignment: &Alignment)
{
    let ratio = draw.draw().get_image_width_per_height(empty);
    stretch(&draw.enforce_aspect_ratio_downscale(ratio, alignment), empty, full, progress)
}

/// Draws a circular progress bar, stretching it over the whole area.
///
/// # Panic
///
/// Panicks if `progress` is not between 0.0 and 1.0.
pub fn stretch<D: ?Sized + Draw>(draw: &DrawContext<D>, empty: &D::ImageResource,
                                 full: &D::ImageResource, progress: f32)
{
    assert!(progress >= 0.0);
    assert!(progress <= 1.0);

    // Drawing the empty image, which serves as a background.
    image::stretch(draw, empty);

    // The top image will be split in 4 rectangles, one for each quater (top-left, top-right,
    // bottom-left, bottom-right). These 4 rectangles are themselves split into two triangles each.
    // By adjusting the positions and uv coordinates of each triangle, we can show a progression.

    // Drawing the top-left triangle of each rectangle.
    for num in 0 .. 4 {
        let local_percent = (progress - 0.25 * num as f32) / 0.125;
        if local_percent <= 0.0 { continue; }
        let local_percent = if local_percent >= 1.0 { 1.0 } else { local_percent };

        let local_matrix = Matrix::translate(1.0, 1.0);
        let local_matrix = Matrix::scale_wh(0.5 * local_percent, 0.5) * local_matrix;
        let local_matrix = Matrix::rotate(num as f32 * -3.141592 * 0.5) * local_matrix;

        let (uv1, uv3) = match num {
            0 => ([0.5, 1.0], [0.5 + 0.5 * local_percent, 1.0]),
            1 => ([1.0, 0.5], [1.0, 0.5 - 0.5 * local_percent]),
            2 => ([0.5, 0.0], [0.5 - 0.5 * local_percent, 0.0]),
            3 => ([0.0, 0.5], [0.0, 0.5 + 0.5 * local_percent]),
            _ => unreachable!()
        };

        draw.draw().draw_triangle(full, &(*draw.matrix() * local_matrix), [uv1, [0.5, 0.5], uv3]);
    }

    // Drawing the bottom-right image of each rectangle.
    for num in 0 .. 4 {
        let local_percent = (progress - 0.125 - 0.25 * num as f32) / 0.125;
        if local_percent <= 0.0 { continue; }
        let local_percent = if local_percent >= 1.0 { 1.0 } else { local_percent };

        let local_matrix = Matrix::translate(1.0, 1.0);
        let local_matrix = Matrix::scale_wh(0.5 * local_percent, 0.5) * local_matrix;
        let local_matrix = Matrix::skew_x(-3.141592 / 4.0) * local_matrix;
        let local_matrix = Matrix::rotate((num + 1) as f32 * -3.141592 * 0.5) * local_matrix;

        let (uv1, uv3) = match num {
            0 => ([1.0, 1.0], [1.0, 1.0 - 0.5 * local_percent]),
            1 => ([1.0, 0.0], [1.0 - 0.5 * local_percent, 0.0]),
            2 => ([0.0, 0.0], [0.0, 0.0 + 0.5 * local_percent]),
            3 => ([0.0, 1.0], [0.0 + 0.5 * local_percent, 1.0]),
            _ => unreachable!()
        };

        draw.draw().draw_triangle(full, &(*draw.matrix() * local_matrix), [uv1, [0.5, 0.5], uv3]);
    }
}

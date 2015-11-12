use Alignment;
use Draw;
use DrawContext;

/// Draws a 9-parts image.
///
/// The image is split into 9 parts: the four corners, the four borders, and the middle. The
/// dimensions are given by the `top_percent`, `right_percent`, `bottom_percent` and
/// `left_percent` parameters.
///
/// The whole context viewport is then filled with this image. The corners will always keep their
/// aspect ratio, the top and bottom borders will be stretched horizontally, the left and
/// right borders will be stretched vertically, and the middle will be stretched.
///
/// The `left_border_percent` parameter is used to determine the percentage of the viewport that
/// should be occupied by the left border of the image. The order borders are automatically
/// calculated by maintaining the correct aspect ratio.
///
/// # Panic
///
/// Panicks if `top_percent + bottom_percent > 1.0` or `left_percent + right_percent > 1.0`.
///
pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, left_border_percent: f32,
                              image_name: &D::ImageResource, top_percent: f32, right_percent: f32,
                              bottom_percent: f32, left_percent: f32)
{
    assert!(top_percent + bottom_percent <= 1.0);
    assert!(left_percent + right_percent <= 1.0);

    let image_width_per_height = draw.draw().get_image_width_per_height(image_name);

    let top_border_percent = left_border_percent * top_percent / left_percent * draw.width_per_height() / image_width_per_height;
    let right_border_percent = top_border_percent * right_percent / top_percent / draw.width_per_height() * image_width_per_height;
    let bottom_border_percent = right_border_percent * bottom_percent / right_percent * draw.width_per_height() / image_width_per_height;

    // top left
    {
        let ctxt = draw.rescale(left_border_percent, top_border_percent, &Alignment::top_left());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [0.0, 1.0], [left_percent, 1.0],
                                  [left_percent, 1.0 - top_percent], [0.0, 1.0 - top_percent]);
    }

    // top right
    {
        let ctxt = draw.rescale(right_border_percent, top_border_percent, &Alignment::top_right());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [1.0 - right_percent, 1.0], [1.0, 1.0],
                                  [1.0, 1.0 - top_percent], [1.0 - right_percent, 1.0 - top_percent]);
    }

    // bottom right
    {
        let ctxt = draw.rescale(right_border_percent, bottom_border_percent, &Alignment::bottom_right());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [1.0 - right_percent, bottom_percent],
                                  [1.0, bottom_percent], [1.0, 0.0], [1.0 - right_percent, 0.0]);
    }

    // bottom left
    {
        let ctxt = draw.rescale(left_border_percent, bottom_border_percent, &Alignment::bottom_left());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [0.0, bottom_percent],
                                  [left_percent, bottom_percent], [left_percent, 0.0], [0.0, 0.0]);
    }

    // top
    {
        let ctxt = draw.rescale(1.0 - left_border_percent - right_border_percent, top_border_percent, &Alignment::top());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [left_percent, 1.0], [1.0 - right_percent, 1.0],
                                  [1.0 - right_percent, 1.0 - top_percent], [left_percent, 1.0 - top_percent]);
    }

    // left
    {
        let ctxt = draw.rescale(left_border_percent, 1.0 - top_border_percent - bottom_border_percent, &Alignment::left());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [0.0, 1.0 - top_percent], [left_percent, 1.0 - top_percent],
                                  [left_percent, bottom_percent], [0.0, bottom_percent]);
    }

    // bottom
    {
        let ctxt = draw.rescale(1.0 - left_border_percent - right_border_percent, bottom_border_percent, &Alignment::bottom());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [left_percent, bottom_percent], [1.0 - right_percent, bottom_percent],
                                  [1.0 - right_percent, 0.0], [left_percent, 0.0]);
    }

    // right
    {
        let ctxt = draw.rescale(right_border_percent, 1.0 - top_border_percent - bottom_border_percent, &Alignment::right());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [1.0 - right_percent, 1.0 - top_percent],
                                  [1.0, 1.0 - top_percent], [1.0, bottom_percent], [1.0 - right_percent, bottom_percent]);
    }

    // middle
    {
        let ctxt = draw.rescale(1.0 - left_border_percent - right_border_percent,
                                 1.0 - top_border_percent - bottom_border_percent,
                                 &Alignment::center());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [left_percent, 1.0 - right_percent],
                                  [1.0 - top_percent, 1.0 - right_percent], [1.0 - top_percent, bottom_percent],
                                  [left_percent, bottom_percent]);
    }
    
    if !draw.cursor_hovered_widget() {
        if draw.is_cursor_hovering() {
            draw.set_cursor_hovered_widget();
        }
    }
}

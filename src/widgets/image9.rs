use Alignment;
use Draw;
use DrawContext;

pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, left_border_percent: f32,
                              image_name: &D::ImageResource, top_percent: f32, right_percent: f32,
                              bottom_percent: f32, left_percent: f32)
{
    assert!(top_percent + bottom_percent <= 1.0);
    assert!(left_percent + right_percent <= 1.0);

    let top_border_percent = left_border_percent * top_percent / left_percent;
    let right_border_percent = top_border_percent * right_percent / top_percent;
    let bottom_border_percent = right_border_percent * bottom_percent / right_percent;

    assert!(top_border_percent + bottom_border_percent <= 1.0);
    assert!(left_border_percent + right_border_percent <= 1.0);

    // top left
    {
        let ctxt = draw.absolute(left_border_percent, top_border_percent, &Alignment::top_left());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [0.0, 1.0], [left_percent, 1.0],
                                  [left_percent, 1.0 - top_percent], [0.0, 1.0 - top_percent]);
    }

    // top right
    {
        let ctxt = draw.absolute(right_border_percent, top_border_percent, &Alignment::top_right());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [1.0 - right_percent, 1.0], [1.0, 1.0],
                                  [1.0, 1.0 - top_percent], [1.0 - right_percent, 1.0 - top_percent]);
    }

    // bottom right
    {
        let ctxt = draw.absolute(right_border_percent, bottom_border_percent, &Alignment::bottom_right());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [1.0 - right_percent, bottom_percent],
                                  [1.0, bottom_percent], [1.0, 0.0], [1.0 - right_percent, 0.0]);
    }

    // bottom left
    {
        let ctxt = draw.absolute(left_border_percent, bottom_border_percent, &Alignment::bottom_left());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [0.0, bottom_percent],
                                  [left_percent, bottom_percent], [left_percent, 0.0], [0.0, 0.0]);
    }

    // top
    {
        let ctxt = draw.absolute(1.0 - left_border_percent - right_border_percent, top_border_percent, &Alignment::top());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [left_percent, 1.0], [1.0 - right_percent, 1.0],
                                  [1.0 - right_percent, 1.0 - top_percent], [left_percent, 1.0 - top_percent]);
    }

    // left
    {
        let ctxt = draw.absolute(left_border_percent, 1.0 - top_border_percent - bottom_border_percent, &Alignment::left());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [0.0, 1.0 - top_percent], [left_percent, 1.0 - top_percent],
                                  [left_percent, bottom_percent], [0.0, bottom_percent]);
    }

    // bottom
    {
        let ctxt = draw.absolute(1.0 - left_border_percent - right_border_percent, bottom_border_percent, &Alignment::bottom());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [left_percent, bottom_percent], [1.0 - right_percent, bottom_percent],
                                  [1.0 - right_percent, 0.0], [left_percent, 0.0]);
    }

    // right
    {
        let ctxt = draw.absolute(right_border_percent, 1.0 - top_border_percent - bottom_border_percent, &Alignment::right());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [1.0 - right_percent, 1.0 - top_percent],
                                  [1.0, 1.0 - top_percent], [1.0, bottom_percent], [1.0 - right_percent, bottom_percent]);
    }

    // middle
    {
        let ctxt = draw.absolute(1.0 - left_border_percent - right_border_percent,
                                 1.0 - top_border_percent - bottom_border_percent,
                                 &Alignment::center());
        draw.draw().draw_image_uv(image_name, ctxt.matrix(), [left_percent, 1.0 - right_percent],
                                  [1.0 - top_percent, 1.0 - right_percent], [1.0 - top_percent, bottom_percent],
                                  [left_percent, bottom_percent]);
    }
}

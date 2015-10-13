use Alignment;
use Draw;
use DrawContext;

pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, normal_image: &D::Resource,
                              hovered_image: &D::Resource, active_image: &D::Resource,
                              alignment: &Alignment)
{
    let ratio = draw.draw().get_image_width_per_height(normal_image);
    stretch(&draw.enforce_aspect_ratio_downscale(ratio, alignment), normal_image, hovered_image, active_image)
}

pub fn stretch<D: ?Sized + Draw>(draw: &DrawContext<D>, normal_image: &D::Resource,
                                 hovered_image: &D::Resource, active_image: &D::Resource)
{
    if draw.is_cursor_hovering() {
        draw.draw().draw_image(hovered_image, draw.matrix());
    } else {
        draw.draw().draw_image(normal_image, draw.matrix());
    }
}

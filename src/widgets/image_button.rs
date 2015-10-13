use Alignment;
use Draw;
use DrawContext;

use widgets::Interaction;

pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, normal_image: &D::ImageResource,
                              hovered_image: &D::ImageResource, active_image: &D::ImageResource,
                              alignment: &Alignment) -> Interaction
{
    let ratio = draw.draw().get_image_width_per_height(normal_image);
    stretch(&draw.enforce_aspect_ratio_downscale(ratio, alignment), normal_image, hovered_image,
                                                 active_image)
}

pub fn stretch<D: ?Sized + Draw>(draw: &DrawContext<D>, normal_image: &D::ImageResource,
                                 hovered_image: &D::ImageResource, active_image: &D::ImageResource)
                                 -> Interaction
{
    if draw.is_cursor_hovering() {
        draw.draw().draw_image(hovered_image, draw.matrix());

        if draw.cursor_was_released() {
            Interaction::Clicked
        } else {
            Interaction::None
        }

    } else {
        draw.draw().draw_image(normal_image, draw.matrix());
        Interaction::None
    }
}

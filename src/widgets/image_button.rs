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
    let widget_id = draw.reserve_widget_id();

    if draw.is_cursor_hovering() {
        draw.set_cursor_hovered_widget();

        if Some(widget_id.clone()) == draw.get_active_widget() {
            draw.draw().draw_image(active_image, draw.matrix());

            if draw.cursor_was_released() {
                draw.clear_active_widget();
                Interaction::Clicked
            } else {
                Interaction::None
            }

        } else if draw.cursor_was_pressed() {
            draw.draw().draw_image(active_image, draw.matrix());
            draw.write_active_widget(widget_id.clone());
            Interaction::None

        } else {
            draw.draw().draw_image(hovered_image, draw.matrix());
            Interaction::None
        }

    } else {
        draw.draw().draw_image(normal_image, draw.matrix());
        Interaction::None
    }
}

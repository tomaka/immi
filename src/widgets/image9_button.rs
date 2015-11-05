use Draw;
use DrawContext;

use widgets::Interaction;
use widgets::image9;

pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, left_border_percent: f32,
                              normal_image: &D::ImageResource, hovered_image: &D::ImageResource,
                              active_image: &D::ImageResource, top_percent: f32, right_percent: f32,
                              bottom_percent: f32, left_percent: f32)
                              -> Interaction
{
    let widget_id = draw.reserve_widget_id();

    if draw.is_cursor_hovering() {
        if Some(widget_id.clone()) == draw.get_active_widget() {
            image9::draw(draw, left_border_percent, active_image, top_percent, right_percent,
                         bottom_percent, left_percent);

            if draw.cursor_was_released() {
                draw.clear_active_widget();
                Interaction::Clicked
            } else {
                Interaction::None
            }

        } else if draw.cursor_was_pressed() {
            image9::draw(draw, left_border_percent, active_image, top_percent, right_percent,
                         bottom_percent, left_percent);
            draw.write_active_widget(widget_id.clone());
            Interaction::None

        } else {
            image9::draw(draw, left_border_percent, hovered_image, top_percent, right_percent,
                         bottom_percent, left_percent);
            Interaction::None
        }

    } else {
        image9::draw(draw, left_border_percent, normal_image, top_percent, right_percent,
                     bottom_percent, left_percent);
        Interaction::None
    }
}

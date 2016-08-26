use Alignment;
use Draw;
use DrawContext;

pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, image_name: &D::ImageResource,
                              alignment: &Alignment)
{
    let draw = draw.animation_stop();

    let ratio = draw.draw().get_image_width_per_height(image_name);
    stretch(&draw.enforce_aspect_ratio_downscale(ratio, alignment), image_name)
}

pub fn stretch<D: ?Sized + Draw>(draw: &DrawContext<D>, image_name: &D::ImageResource) {
    if !draw.cursor_hovered_widget() {
        if draw.is_cursor_hovering() {
            draw.set_cursor_hovered_widget();
        }
    }

    draw.draw().draw_image(image_name, &draw.matrix());
}

/// Increases the size of the image until it covers the context, and draws it.
pub fn cover<D: ?Sized + Draw>(draw: &DrawContext<D>, image_name: &D::ImageResource,
                              alignment: &Alignment)
{
    let draw = draw.animation_stop();
    let ratio = draw.draw().get_image_width_per_height(image_name);
    stretch(&draw.enforce_aspect_ratio_upscale(ratio, alignment), image_name)
}

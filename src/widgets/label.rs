use Alignment;
use Draw;
use DrawContext;
use HorizontalAlignment;

pub fn contain<D: ?Sized + Draw>(draw: &DrawContext<D>, font: &D::FontResource, text: &str,
                                 alignment: &Alignment, rgb_color: [f32; 3])
{
    let ratio = draw.draw().get_text_width_per_em(font, text);
    
    let draw = draw.enforce_aspect_ratio_downscale(ratio, alignment);

    if !draw.cursor_hovered_widget() {
        if draw.is_cursor_hovering() {
            draw.set_cursor_hovered_widget();
        }
    }

    draw.draw().draw_text(font, &draw.matrix(), text, rgb_color);
}

pub fn cover<D: ?Sized + Draw>(draw: &DrawContext<D>, font: &D::FontResource, text: &str,
                               alignment: &Alignment, rgb_color: [f32; 3])
{
    let ratio = draw.draw().get_text_width_per_em(font, text);
    
    let draw = draw.enforce_aspect_ratio_upscale(ratio, alignment);

    if !draw.cursor_hovered_widget() {
        if draw.is_cursor_hovering() {
            draw.set_cursor_hovered_widget();
        }
    }

    draw.draw().draw_text(font, &draw.matrix(), text, rgb_color);
}

/// The text will use the current height and will stretch horizontally as needed to preserve the
/// correct aspect ratio.
pub fn flow<D: ?Sized + Draw>(draw: &DrawContext<D>, font: &D::FontResource, text: &str,
                              alignment: &HorizontalAlignment, rgb_color: [f32; 3])
{
    let ratio = draw.draw().get_text_width_per_em(font, text);

    let current_width_per_height = draw.width_per_height();
    let draw = draw.horizontal_rescale(ratio / current_width_per_height, &alignment);

    if !draw.cursor_hovered_widget() {
        if draw.is_cursor_hovering() {
            draw.set_cursor_hovered_widget();
        }
    }

    draw.draw().draw_text(font, &draw.matrix(), text, rgb_color);
}

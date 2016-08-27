// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Image buttons are images that are clickable.
//!
//! You can specify different images for when the button is in a normal state, a hovered state,
//! or an active state.
//!
//! All the functions in this module return an `Interaction` object that indicates whether they
//! were clicked. 

use Alignment;
use Draw;
use DrawContext;
use UiState;

use widgets::Interaction;

/// Same as `image::draw`, except that the image is clickable.
pub fn draw<D: ?Sized + Draw>(draw: &DrawContext<D>, ui_state: &mut UiState,
                              normal_image: &D::ImageResource, hovered_image: &D::ImageResource,
                              active_image: &D::ImageResource, alignment: &Alignment)
                              -> Interaction
{
    let draw = draw.animation_stop();
    let ratio = draw.draw().get_image_width_per_height(normal_image);
    stretch(&draw.enforce_aspect_ratio_downscale(ratio, alignment), ui_state, normal_image,
            hovered_image, active_image)
}

/// Same as `image::stretch`, except that the image is clickable.
pub fn stretch<D: ?Sized + Draw>(draw: &DrawContext<D>, ui_state: &mut UiState,
                                 normal_image: &D::ImageResource, hovered_image: &D::ImageResource,
                                 active_image: &D::ImageResource) -> Interaction
{
    let widget_id = draw.reserve_widget_id();

    if draw.is_cursor_hovering() {
        draw.set_cursor_hovered_widget();

        if Some(widget_id.clone()) == ui_state.active_widget {
            draw.draw().draw_image(active_image, &draw.matrix());

            if draw.cursor_was_released() {
                ui_state.active_widget = None;
                Interaction::Clicked
            } else {
                Interaction::None
            }

        } else if draw.cursor_was_pressed() {
            draw.draw().draw_image(active_image, &draw.matrix());
            ui_state.active_widget = Some(widget_id.clone());
            Interaction::None

        } else {
            draw.draw().draw_image(hovered_image, &draw.matrix());
            Interaction::None
        }

    } else {
        draw.draw().draw_image(normal_image, &draw.matrix());
        Interaction::None
    }
}

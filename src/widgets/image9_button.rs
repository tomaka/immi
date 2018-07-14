// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Same as `image9`, except that the image is clickable.

use DrawImage;
use DrawContext;
use UiState;

use widgets::Interaction;
use widgets::image9;

/// Same as `image9::draw`, except that the image is clickable. You can specify different images
/// for when the button is non-hovered, hovered, or active. 
pub fn draw<D: ?Sized + DrawImage<I>, I: ?Sized>(draw: &DrawContext<D>, ui_state: &mut UiState,
                                                 left_border_percent: f32, normal_image: &I,
                                                 hovered_image: &I, active_image: &I,
                                                 top_percent: f32, right_percent: f32, bottom_percent: f32,
                                                 left_percent: f32) -> Interaction
{
    let widget_id = draw.reserve_widget_id();

    if draw.is_cursor_hovering() {
        if Some(widget_id.clone()) == ui_state.active_widget {
            image9::draw(draw, left_border_percent, active_image, top_percent, right_percent,
                         bottom_percent, left_percent);

            if draw.cursor_was_released() {
                ui_state.active_widget = None;
                Interaction::Clicked
            } else {
                Interaction::None
            }

        } else if draw.cursor_was_pressed() {
            image9::draw(draw, left_border_percent, active_image, top_percent, right_percent,
                         bottom_percent, left_percent);
            ui_state.active_widget = Some(widget_id.clone());
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

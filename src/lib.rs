// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Immediate mode UI and general application architecture
//!
//! The principle of an immediate mode UI is that the position and dimensions of the UI elements
//! are calculated at each frame. The current state of the user interface (for example the content
//! of text boxes, whether a checkbox is checked, which tab is the current tab, etc.) is not stored
//! by the immi library, but in a user-defined structure.
//!
//! There are three steps involved to make your program work with immi:
//!
//! - Create a custom structure that describes the state of your user interface. This state
//!   should contain a `immi::UiState` object.
//! - Create a type that implements the `immi::Draw` trait and that handles loading images,
//!   fonts, and drawing the user interface on the screen. This is the most complicated part.
//! - Create a function whose purpose is to draw your user interface. Usually you want to
//!   create one function for each part of the UI instead, and call all of them in a main function.
//!
//! At each frame, when it is time to draw your UI:
//!
//! - Call `immi::draw`. You will get a `SharedDrawContext`. This object represents a context for
//!   drawing the entirety of your UI.
//! - Call `draw()` on your `SharedDrawContext` in order to obtain a `DrawContext`. You will need
//!   to pass your implementation of `immi::Draw` (see above), indicate the position of the mouse
//!   pointer, the dimensions of the viewport, and whether or not the main mouse button was pressed
//!   or released. 
//! - Call your custom UI-drawing function (see above), and pass it a reference to the `DrawContext`
//!   and a mutable reference to your custom state-holding structure.
//! - The function draws the various elements and updates the UI state.
//!
//! ## Example
//!
//! ```rust
//! // Object that will allow you to draw the UI.
//! struct MyDrawer;
//! impl immi::Draw for MyDrawer {
//!     type ImageResource = str;
//!     type TextStyle = str;
//!
//!     fn draw_triangle(&mut self, _: &str, _: &immi::Matrix, _: [[f32; 2]; 3]) {}
//!     fn get_image_width_per_height(&mut self, _: &str) -> f32 { 1.0 }
//!     fn draw_glyph(&mut self, _: &str, _: char, _: &immi::Matrix) { }
//!     fn line_height(&self, _: &str) -> f32 { 1.2 }
//!     fn kerning(&self, _: &str, _: char, _: char) -> f32 { 0.0 }
//!     fn glyph_infos(&self, _: &str, _: char) -> immi::GlyphInfos {
//!         immi::GlyphInfos { width: 1.0, height: 1.0, x_offset: 0.0,
//!                            y_offset: 1.0, x_advance: 1.0 }
//!     }
//! }
//!
//! struct MyUiState {
//!     immi_state: immi::UiState,  
//!     widget1_text: String,
//!     checkbox: bool,
//! }
//!
//! fn draw_ui(ctxt: &immi::DrawContext<MyDrawer>, ui_state: &mut MyUiState) {
//!     // ...
//! }
//!
//! let mut my_state = MyUiState { widget1_text: String::new(), checkbox: false,
//!                                immi_state: Default::default() };
//! let mut drawer = MyDrawer;
//! 
//! loop {
//!     let ui_context = immi::draw();
//!     let ui_context = ui_context.draw(1024.0, 768.0, &mut drawer, None, false, false);
//!     draw_ui(&ui_context, &mut my_state);
//! # break;
//! }
//! ```
//!
//! # Drawing
//!
//! Once you have a `DrawContext` you can start drawing your user interface.
//!
//! A `DrawContext` represents an area of the viewport where things you should be drawn. Initially
//! this area contain the whole viewport, but you can call methods on the `DrawContext` to adjust
//! this area.
//!
//! In order to draw widgets, you can use the functions provided by the modules of the `widgets`
//! module of this library.
//!
//! Example:
//!
//! ```rust
//! fn draw_ui<D>(ctxt: &immi::DrawContext<D>)
//!     where D: immi::Draw<ImageResource = str>
//! {
//!     // Assuming you immediately called `draw_ui` after creating the `DrawContext`, the `ctxt`
//!     // object represents the whole viewport..
//!
//!     // Draws an image on the whole viewport.
//!     // The bottom alignment is used if the aspect ratio of the image doesn't match the aspect
//!     // ratio of the viewport.
//!     // The "background" string will be passed to your implementation of `immi::Draw`, so you
//!     // are free to choose what the type of data exactly is.
//!     immi::widgets::image::draw(ctxt, "background", &immi::Alignment::bottom());
//!
//!     // We resize the viewport so that it only covers the top half of the screen
//!     let ctxt = ctxt.vertical_rescale(0.5, &immi::VerticalAlignment::Bottom);
//!     draw_bottom_bar(&ctxt);
//! }
//!
//! fn draw_bottom_bar<D>(ctxt: &immi::DrawContext<D>)
//!     where D: immi::Draw<ImageResource = str>
//! {
//!     // Draws an image on the bottom half of the screen
//!     immi::widgets::image::draw(ctxt, "top_background", &immi::Alignment::center());
//! }
//! ```
//!
pub use draw::Draw;
pub use draw::GlyphInfos;
pub use id::WidgetId;
pub use layout::draw;
pub use layout::Alignment;
pub use layout::DrawContext;
pub use layout::SharedDrawContext;
pub use layout::HorizontalAlignment;
pub use layout::VerticalAlignment;
pub use matrix::Matrix;

mod draw;
mod id;
mod layout;
mod matrix;

pub mod animations;
pub mod widgets;

/// Contains some persistent info about the UI.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct UiState {
    /// Identifier of the widget that is currently active.
    ///
    /// For example if you maintain the left button of the mouse, the element under will be active.
    /// If you then move your mouse somewhere else, the active element doesn't change.
    pub active_widget: Option<WidgetId>,
}

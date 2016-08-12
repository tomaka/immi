//! # Immediate mode UI and general application architecture
//!
//! The principle of immediate more UI is that the position and dimensions of the UI elements are
//! calculated at each frame. The current state of the user interface (for example the content of
//! text boxes, if there are multiple tabs which one is the current one, etc.) is not stored within
//! widgets themselves, but in an external structure.
//! 
//! Here is how you design your code in order to work with immi:
//! 
//! - You define a custom structure that describes the state of your user interface.
//! - You define a type that implements the `immi::Draw` trait.
//! - You define a function whose purpose is to draw your user interface. Usually you want to
//!   create one function for each part of the UI instead, and call all of them in a main function.
//! 
//! There are only two objects that you store persistently:
//! 
//! - An instance of your custom struct that describes your UI.
//! - An instance of `immi::UiState`.
//! 
//! These two objects describe your user interface. They are the only state that is required.
//! 
//! At each frame, when it is time to draw your UI, you:
//! 
//! - Call `immi::draw` and pass a reference to your `immi::UiState`. You will get
//!   a `SharedDrawContext`. This object represents a context for drawing the entirety of your UI.
//! - Sometimes you need to share information between multiple user interfaces. For example in a
//!   game you sometimes have a main UI, but also small in-game overlays. A shared draw context
//!   shares data between all of these. Call `draw()` on your `SharedDrawContext` in order to
//!   obtain a `DrawContext`. You will need to pass your drawing object.
//! - Once you have a `DrawContext`, call your custom UI-drawing function, and pass the
//!   `DrawContext` by reference and your custom UI-state struct by mutable reference.
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
//!     fn draw_text(&mut self, _: &str, _: &immi::Matrix, _: &str) { }
//!     fn get_text_width_per_em(&mut self, _: &str, _: &str) -> f32 { 1.0 }
//! }
//! 
//! struct MyUiState {
//!     widget1_text: String,
//!     checkbox: bool,
//! }
//! 
//! fn draw_ui(ctxt: &immi::DrawContext<MyDrawer>, ui_state: &mut MyUiState) {
//!     // ...
//! }
//! 
//! let mut my_state = MyUiState { widget1_text: String::new(), checkbox: false };
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
//! Example:
//!
//! ```rust
//! fn draw_ui<D>(draw: immi::DrawContext<D>) where D: immi::Draw<ImageResource = str> {
//!     // draws an image on the whole screen
//!     // the bottom alignment is used if the aspect ratio of the image doesn't match the aspect
//!     // ratio of the viewport
//!     immi::widgets::image::draw(&draw, "background", &immi::Alignment::bottom());
//!
//!     // we resize the viewport so that it only covers the top half of the screen
//!     let draw = draw.vertical_rescale(0.5, &immi::VerticalAlignment::Top);
//!
//!     // draws an image on the top half of the screen
//!     immi::widgets::image::draw(&draw, "top_background", &immi::Alignment::center());
//! }
//! ```
//!
pub use draw::Draw;
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

//!
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
extern crate time;

pub use draw::Draw;
pub use id::WidgetId;
pub use layout::Alignment;
pub use layout::DrawContext;
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

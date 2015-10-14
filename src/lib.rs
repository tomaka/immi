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
#[macro_use]
extern crate lazy_static;
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

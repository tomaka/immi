pub mod circular_progress_bar;
pub mod image;
pub mod image9;
pub mod image_button;
pub mod image9_button;
pub mod label;
pub mod progress_bar;

/// Whether the cursor clicked on the widget.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum Interaction {
    /// The cursor clicked.
    Clicked,
    /// The cursor didn't click.
    None,
}

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use time;

use Draw;
use Matrix;
use WidgetId;

use animations::Animation;
use animations::Interpolation;

/// Contains everything required to draw a widget.
pub struct DrawContext<'a, D: ?Sized + Draw + 'a> {
    /// Data that is shared between the draw context and all its children.
    shared: Arc<Shared<'a, D>>,

    matrix: Matrix,
    width: f32,
    height: f32,

    /// Position of the cursor between `-1.0` and `1.0`, where -1.0 is the left or bottom, and 1.0
    /// is the right or top of the window.
    ///
    /// This is the position of the cursor in the original viewport, not in the *current* viewport.
    cursor: Option<[f32; 2]>,

    cursor_was_pressed: bool,
    cursor_was_released: bool,
}

struct Shared<'a, D: ?Sized + Draw + 'a> {
    draw: Mutex<&'a mut D>,
    active_widget: Mutex<&'a mut Option<WidgetId>>,
    next_widget_id: AtomicUsize,

    /// True if the cursor is over an element of the UI.
    cursor_hovered_widget: AtomicBool,
}

impl<'a, D: ?Sized + Draw + 'a> DrawContext<'a, D> {
    // TODO: change this
    /// UNSTABLE, WILL BE CHANGED
    pub fn start(width: f32, height: f32, draw: &'a mut D, cursor: Option<[f32; 2]>,
                 cursor_was_pressed: bool, cursor_was_released: bool,
                 active_widget: &'a mut Option<WidgetId>) -> DrawContext<'a, D>
    {
        DrawContext {
            matrix: Matrix::identity(),
            width: width,
            height: height,
            cursor: cursor,
            cursor_was_pressed: cursor_was_pressed,
            cursor_was_released: cursor_was_released,
            shared: Arc::new(Shared {
                draw: Mutex::new(draw),
                active_widget: Mutex::new(active_widget),
                next_widget_id: AtomicUsize::new(1),
                cursor_hovered_widget: AtomicBool::new(false),
            }),
        }
    }

    /// UNSTABLE. Obtains the underlying `draw` object.
    #[inline]
    pub fn draw(&self) -> MutexGuard<&'a mut D> {
        self.shared.draw.lock().unwrap()
    }

    #[inline]
    pub fn matrix(&self) -> &Matrix {
        &self.matrix
    }

    /// Returns true if the cursor went from up to down in the current frame.
    ///
    /// This is the value that was passed when constructing the context.
    #[inline]
    pub fn cursor_was_pressed(&self) -> bool {
        self.cursor_was_pressed
    }

    /// Returns true if the cursor went from down to up in the current frame.
    ///
    /// This is the value that was passed when constructing the context.
    #[inline]
    pub fn cursor_was_released(&self) -> bool {
        self.cursor_was_released
    }

    /// Returns true if one of the elements that has been drawn is under the mouse cursor.
    ///
    /// When you create the context, this value is initally false. Each widget that you draw can
    /// call `set_cursor_hovered_widget` to pass this value to true.
    #[inline]
    pub fn cursor_hovered_widget(&self) -> bool {
        self.shared.cursor_hovered_widget.load(Ordering::Relaxed)
    }

    /// Signals the context that the cursor is currently hovering it. This can be later retreived
    /// with `cursor_hovered_widget()`.
    #[inline]
    pub fn set_cursor_hovered_widget(&self) {
        self.shared.cursor_hovered_widget.store(true, Ordering::Relaxed);
    }

    #[inline]
    pub fn reserve_widget_id(&self) -> WidgetId {
        self.shared.next_widget_id.fetch_add(1, Ordering::Relaxed).into()
    }

    #[inline]
    pub fn get_active_widget(&self) -> Option<WidgetId> {
        self.shared.active_widget.lock().unwrap().clone()
    }

    #[inline]
    pub fn write_active_widget(&self, id: WidgetId) {
        **self.shared.active_widget.lock().unwrap() = Some(id);
    }

    #[inline]
    pub fn clear_active_widget(&self) {
        **self.shared.active_widget.lock().unwrap() = None;
    }

    /// Returns true if the cursor is currently hovering this part of the viewport.
    #[inline]
    pub fn is_cursor_hovering(&self) -> bool {
        /// Calculates whether the point is in a rectangle multiplied by a matrix.
        fn test(matrix: &Matrix, point: &[f32; 2]) -> bool {
            // We start by calculating the positions of the four corners of the shape in viewport
            // coordinates, so that they can be compared with the point which is already in
            // viewport coordinates.

            let top_left = *matrix * [-1.0, 1.0, 1.0];
            let top_left = [top_left[0] / top_left[2], top_left[1] / top_left[2]];

            let top_right = *matrix * [1.0, 1.0, 1.0];
            let top_right = [top_right[0] / top_right[2], top_right[1] / top_right[2]];

            let bot_left = *matrix * [-1.0, -1.0, 1.0];
            let bot_left = [bot_left[0] / bot_left[2], bot_left[1] / bot_left[2]];

            let bot_right = *matrix * [1.0, -1.0, 1.0];
            let bot_right = [bot_right[0] / bot_right[2], bot_right[1] / bot_right[2]];

            // The point is within our rectangle if and only if it is on the right side of each
            // border of the rectangle (taken in the right order).
            //
            // To check this, we calculate the dot product of the vector `point - corner` with
            // `next_corner - corner`. If the value is positive, then the angle is inferior to
            // 90°. If the the value is negative, the angle is superior to 90° and we know that
            // the cursor is outside of the rectangle.

            if (point[0] - top_left[0]) * (top_right[0] - top_left[0]) +
               (point[1] - top_left[1]) * (top_right[1] - top_left[1]) < 0.0
            {
                return false;
            }

            if (point[0] - top_right[0]) * (bot_right[0] - top_right[0]) +
               (point[1] - top_right[1]) * (bot_right[1] - top_right[1]) < 0.0
            {
                return false;
            }

            if (point[0] - bot_right[0]) * (bot_left[0] - bot_right[0]) +
               (point[1] - bot_right[1]) * (bot_left[1] - bot_right[1]) < 0.0
            {
                return false;
            }

            if (point[0] - bot_left[0]) * (top_left[0] - bot_left[0]) +
               (point[1] - bot_left[1]) * (top_left[1] - bot_left[1]) < 0.0
            {
                return false;
            }

            true
        }

        if let Some(cursor) = self.cursor {
            test(self.matrix(), &cursor)
        } else {
            false
        }
    }

    /// Returns the ratio of the width of the surface divided by its height.
    #[inline]
    pub fn width_per_height(&self) -> f32 {
        self.width / self.height
    }

    /// Builds a new draw context containing a subpart of the current context, but with a margin.
    ///
    /// The margin is expressed in percentage of the surface (between 0.0 and 1.0).
    #[inline]
    pub fn margin(&self, top: f32, right: f32, bottom: f32, left: f32) -> DrawContext<'a, D> {
        // TODO: could be more efficient
        self.rescale(1.0 - left, 1.0 - top, &Alignment::bottom_right())
            .rescale(1.0 - right, 1.0 - bottom, &Alignment::top_left())
    }

    /// Builds a new draw context containing a subpart of the current context, but with a margin.
    ///
    /// If the width of the surface is inferior to the height then the margin is expressed as a
    /// percentage of the width, and vice versa.
    ///
    /// This guarantees that the size in pixels of the margin is the same if you pass the same
    /// values.
    #[inline]
    pub fn uniform_margin(&self, top: f32, right: f32, bottom: f32, left: f32)
                          -> DrawContext<'a, D>
    {
        let wph = self.width_per_height();
        let wph = if wph < 1.0 { 1.0 } else { wph };

        let hpw = 1.0 / self.width_per_height();
        let hpw = if hpw < 1.0 { 1.0 } else { hpw };

        self.margin(top / hpw, right / wph, bottom / hpw, left / wph)
    }

    /// Modifies the layout so that the given width per height ratio is respected. The size of the
    /// new viewport will always been equal or small to the existing viewport.
    ///
    /// If the viewport needs to be reduced horizontally, then the horizontal alignment is used. If
    /// it needs to be reduced vertically, then the vertical alignment is used.
    pub fn enforce_aspect_ratio_downscale(&self, width_per_height: f32, alignment: &Alignment)
                                          -> DrawContext<'a, D>
    {
        let current_width_per_height = self.width_per_height();

        if width_per_height > current_width_per_height {
            let alignment = alignment.vertical;
            self.vertical_rescale(current_width_per_height / width_per_height, &alignment)

        } else {
            let alignment = alignment.horizontal;
            self.horizontal_rescale(width_per_height / current_width_per_height, &alignment)
        }
    }

    /// Modifies the layout so that the given width per height ratio is respected. The size of the
    /// new viewport will always been equal or greater to the existing viewport.
    ///
    /// If the viewport needs to be increased horizontally, then the horizontal alignment is used.
    /// If it needs to be increased vertically, then the vertical alignment is used.
    pub fn enforce_aspect_ratio_upscale(&self, width_per_height: f32, alignment: &Alignment)
                                        -> DrawContext<'a, D>
    {
        let current_width_per_height = self.width_per_height();

        if width_per_height > current_width_per_height {
            let alignment = alignment.horizontal;
            self.horizontal_rescale(width_per_height / current_width_per_height, &alignment)

        } else {
            let alignment = alignment.vertical;
            self.vertical_rescale(current_width_per_height / width_per_height, &alignment)
        }
    }

    /// Builds a new draw context containing a subpart of the current context. The width of the new
    /// viewport will be the same as the current one, but its new height will be multipled by
    /// the value of `scale`.
    ///
    /// The alignment is used to determine the position of the new viewport inside the old one.
    #[inline]
    pub fn vertical_rescale(&self, scale: f32, alignment: &VerticalAlignment)
                            -> DrawContext<'a, D>
    {
        let y = match alignment {
            &VerticalAlignment::Center => 0.0,
            &VerticalAlignment::Bottom => scale - 1.0,
            &VerticalAlignment::Top => 1.0 - scale,
        };

        DrawContext {
            matrix: self.matrix * Matrix::translate(0.0, y) * Matrix::scale_wh(1.0, scale),
            width: self.width,
            height: self.height * scale,
            shared: self.shared.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }

    /// Builds a new draw context containing a subpart of the current context. The height of the new
    /// viewport will be the same as the current one, but its new width will be multipled by
    /// the value of `scale`.
    ///
    /// The alignment is used to determine the position of the new viewport inside the old one.
    #[inline]
    pub fn horizontal_rescale(&self, scale: f32, alignment: &HorizontalAlignment)
                              -> DrawContext<'a, D>
    {
        let x = match alignment {
            &HorizontalAlignment::Center => 0.0,
            &HorizontalAlignment::Left => scale - 1.0,
            &HorizontalAlignment::Right => 1.0 - scale,
        };

        DrawContext {
            matrix: self.matrix * Matrix::translate(x, 0.0) * Matrix::scale_wh(scale, 1.0),
            width: self.width * scale,
            height: self.height,
            shared: self.shared.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }

    /// Splits the viewport in `splits` vertical chunks of equal size.
    // TODO: don't return a Vec
    #[inline]
    pub fn vertical_split(&self, splits: usize) -> Vec<DrawContext<'a, D>> {
        // we use a "real" function because closures don't implement Clone
        #[inline(always)] fn gen(_: usize) -> f32 { 1.0 }
        self.vertical_split_weights((0 .. splits).map(gen as fn(usize) -> f32))
    }

    /// Same as `vertical_split`, but attributes a weight to each chunk. For example a chunk of
    /// weight 2 will have twice the size of a chunk of weight 1.
    // TODO: don't return a Vec
    #[inline]
    pub fn vertical_split_weights<I>(&self, weights: I) -> Vec<DrawContext<'a, D>>
                                     where I: ExactSizeIterator<Item = f32> + Clone
    {
        self.split_weights(weights, true)
    }

    /// Splits the viewport in `splits` horizontal chunks of equal size.
    // TODO: don't return a Vec
    #[inline]
    pub fn horizontal_split(&self, splits: usize) -> Vec<DrawContext<'a, D>> {
        // we use a "real" function because closures don't implement Clone
        #[inline(always)] fn gen(_: usize) -> f32 { 1.0 }
        self.horizontal_split_weights((0 .. splits).map(gen as fn(usize) -> f32))
    }

    /// Same as `horizontal_split`, but attributes a weight to each chunk. For example a chunk of
    /// weight 2 will have twice the size of a chunk of weight 1.
    // TODO: don't return a Vec
    #[inline]
    pub fn horizontal_split_weights<I>(&self, weights: I) -> Vec<DrawContext<'a, D>>
                                       where I: ExactSizeIterator<Item = f32> + Clone
    {
        self.split_weights(weights, false)
    }

    /// Internal implementation of the split functions.
    // TODO: don't return a Vec
    fn split_weights<I>(&self, weights: I, vertical: bool) -> Vec<DrawContext<'a, D>>
                        where I: ExactSizeIterator<Item = f32> + Clone
    {
        assert!(weights.len() != 0);

        let total_weight = weights.clone().fold(0.0, |a, b| a + b);
        let total_weight_inverse = 1.0 / total_weight;

        let mut current_offset = 0.0;

        weights.map(|weight| {
            let new_width = if !vertical { self.width * weight * total_weight_inverse } else { self.width };
            let new_height = if vertical { self.height * weight * total_weight_inverse } else { self.height };

            let scale_matrix = if vertical {
                Matrix::scale_wh(1.0, weight * total_weight_inverse)
            } else {
                Matrix::scale_wh(weight * total_weight_inverse, 1.0)
            };

            let pos_matrix = if vertical {
                let y = 1.0 - 2.0 * (current_offset + weight * 0.5) * total_weight_inverse;
                Matrix::translate(0.0, y)
            } else {
                let x = 2.0 * (current_offset + weight * 0.5) * total_weight_inverse - 1.0;
                Matrix::translate(x, 0.0)
            };

            current_offset += weight;

            DrawContext {
                matrix: self.matrix * pos_matrix * scale_matrix,
                width: new_width,
                height: new_height,
                shared: self.shared.clone(),
                cursor: self.cursor,
                cursor_was_pressed: self.cursor_was_pressed,
                cursor_was_released: self.cursor_was_released,
            }
        }).collect()
    }

    /// Changes the dimensions of the context.
    ///
    /// The dimensions are a percentage of the current dimensions. For example to divide the width
    /// by two, you need to pass `0.5`.
    ///
    /// The alignment is used to determine the position of the newly-created context relative to
    /// the old one.
    pub fn rescale(&self, width_percent: f32, height_percent: f32, alignment: &Alignment)
                   -> DrawContext<'a, D>
    {
        let x = match alignment.horizontal {
            HorizontalAlignment::Center => 0.0,
            HorizontalAlignment::Left => width_percent - 1.0,
            HorizontalAlignment::Right => 1.0 - width_percent,
        };

        let y = match alignment.vertical {
            VerticalAlignment::Center => 0.0,
            VerticalAlignment::Bottom => height_percent - 1.0,
            VerticalAlignment::Top => 1.0 - height_percent,
        };

        DrawContext {
            matrix: self.matrix * Matrix::translate(x, y)
                                * Matrix::scale_wh(width_percent, height_percent),
            width: self.width * width_percent,
            height: self.height * height_percent,
            shared: self.shared.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }

    pub fn animate<A, I>(&self, animation: A, interpolation: I, start_time: u64,
                         duration_ns: u64) -> DrawContext<'a, D>
        where A: Animation, I: Interpolation
    {
        let now = time::precise_time_ns();

        let interpolation = interpolation.calculate(now, start_time, duration_ns);
        let matrix = animation.animate(interpolation);

        DrawContext {
            matrix: self.matrix * matrix,
            width: self.width,
            height: self.height,
            shared: self.shared.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }
}

impl<'a, D: ?Sized + Draw + 'a> Clone for DrawContext<'a, D> {
    fn clone(&self) -> DrawContext<'a, D> {
        DrawContext {
            matrix: self.matrix.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            shared: self.shared.clone(),
            cursor: self.cursor.clone(),
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }
}

/// Represents the alignment of a viewport.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Alignment {
    /// The horizontal alignment.
    pub horizontal: HorizontalAlignment,
    /// The vertical alignment.
    pub vertical: VerticalAlignment,
}

impl Alignment {
    /// Shortcut for `(center, center)`.
    #[inline]
    pub fn center() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Center,
        }
    }

    /// Shortcut for `(center, bottom)`.
    #[inline]
    pub fn bottom() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Bottom,
        }
    }

    /// Shortcut for `(center, top)`.
    #[inline]
    pub fn top() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Top,
        }
    }

    /// Shortcut for `(right, center)`.
    #[inline]
    pub fn right() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Right,
            vertical: VerticalAlignment::Center,
        }
    }

    /// Shortcut for `(left, center)`.
    #[inline]
    pub fn left() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Left,
            vertical: VerticalAlignment::Center,
        }
    }

    /// Shortcut for `(left, top)`.
    #[inline]
    pub fn top_left() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Left,
            vertical: VerticalAlignment::Top,
        }
    }

    /// Shortcut for `(right, top)`.
    #[inline]
    pub fn top_right() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Right,
            vertical: VerticalAlignment::Top,
        }
    }

    /// Shortcut for `(right, bottom)`.
    #[inline]
    pub fn bottom_right() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Right,
            vertical: VerticalAlignment::Bottom,
        }
    }

    /// Shortcut for `(left, bottom)`.
    #[inline]
    pub fn bottom_left() -> Alignment {
        Alignment {
            horizontal: HorizontalAlignment::Left,
            vertical: VerticalAlignment::Bottom,
        }
    }
}

/// Describes a horizontal alignment.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HorizontalAlignment {
    /// Align in the middle.
    Center,
    /// Align left.
    Left,
    /// Align right.
    Right,
}

/// Describes a vertical alignment.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Align in the middle.
    Center,
    /// Align top.
    Top,
    /// Align bottom.
    Bottom,
}

// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::cell::Cell;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::SystemTime;

use Draw;
use Matrix;
use WidgetId;

use animations::Interpolation;

/// Start drawing your UI.
///
/// This function builds a `SharedDrawContext` that can be used to build `DrawContext`s.
pub fn draw() -> SharedDrawContext {
    SharedDrawContext {
        shared1: Arc::new(Shared1 {
            next_widget_id: AtomicUsize::new(1),
            cursor_hovered_widget: AtomicBool::new(false),
        })
    }
}

/// A context shared between all draw contexts.
pub struct SharedDrawContext {
    shared1: Arc<Shared1>,
}

impl SharedDrawContext {
    ///
    /// The cursor coordinates, if any, must be in OpenGL viewport coordinates. In other words,
    /// `[-1.0, -1.0]` corresponds to the bottom-left hand corner of the screen, and `[1.0, 1.0]`
    /// to the top-right hand corner.
    pub fn draw<'b, D: ?Sized + Draw + 'b>(&self, width: f32, height: f32, draw: &'b mut D,
                                           cursor: Option<[f32; 2]>, cursor_was_pressed: bool,
                                           cursor_was_released: bool) -> DrawContext<'b, D>
    {
        DrawContext {
            matrix: Matrix::identity(),
            width: width,
            height: height,
            animation: None,
            cursor: cursor,
            cursor_was_pressed: cursor_was_pressed,
            cursor_was_released: cursor_was_released,
            shared1: self.shared1.clone(),
            shared2: Rc::new(Shared2 {
                draw: RefCell::new(draw),
                cursor_hovered_widget: Cell::new(false),
            }),
        }
    }

    /// Returns true if one of the elements that has been drawn by one of the draw contexts was
    /// under the mouse cursor.
    ///
    /// This function can be used to determine whether the user is hovering some part of the UI.
    /// In other words, if this function returns false, then you know that the user is hovering
    /// what is under the UI.
    ///
    /// When you create the context, this value is initally false. Each widget that you draw can
    /// call `set_cursor_hovered_widget` to pass this value to true.
    #[inline]
    pub fn cursor_hovered_widget(&self) -> bool {
        self.shared1.cursor_hovered_widget.load(Ordering::Relaxed)
    }
}

struct Shared1 {
    next_widget_id: AtomicUsize,
    cursor_hovered_widget: AtomicBool,
}

/// Contains everything required to draw a widget.
pub struct DrawContext<'b, D: ?Sized + Draw + 'b> {
    shared1: Arc<Shared1>,
    shared2: Rc<Shared2<'b, D>>,

    matrix: Matrix,
    width: f32,
    height: f32,

    /// If `Some`, contains the base animation. The first value is the matrix of the start of
    /// the animation, and the second value is the percentage of the linear interpolation between
    /// `0.0` and `1.0`.
    animation: Option<(Matrix, f32)>,

    /// Position of the cursor between `-1.0` and `1.0`, where -1.0 is the left or bottom, and 1.0
    /// is the right or top of the window.
    ///
    /// This is the position of the cursor in the original viewport, not in the *current* viewport.
    cursor: Option<[f32; 2]>,

    cursor_was_pressed: bool,
    cursor_was_released: bool,
}

struct Shared2<'a, D: ?Sized + Draw + 'a> {
    draw: RefCell<&'a mut D>,

    /// True if the cursor is over an element of the UI.
    cursor_hovered_widget: Cell<bool>,
}

impl<'b, D: ?Sized + Draw + 'b> DrawContext<'b, D> {
    /// UNSTABLE. Obtains the underlying `draw` object.
    #[inline]
    #[doc(hidden)]
    pub fn draw(&self) -> RefMut<&'b mut D> {
        self.shared2.draw.borrow_mut()
    }

    /// Returns a matrix that turns a fullscreen rectangle into a rectangle that covers only the
    /// context's area.
    #[inline]
    pub fn matrix(&self) -> Matrix {
        if let Some((matrix, percent)) = self.animation {
            // TODO: correct decomposition with https://drafts.csswg.org/css-transforms/#decomposing-a-2d-matrix

            #[inline]
            fn lerp(a: f32, b: f32, f: f32) -> f32 { a + (b - a) * f }

            let matrix = matrix.0;
            let my_m = self.matrix.0;

            Matrix([
                [lerp(matrix[0][0], my_m[0][0], percent),  lerp(matrix[0][1], my_m[0][1], percent)],
                [lerp(matrix[1][0], my_m[1][0], percent),  lerp(matrix[1][1], my_m[1][1], percent)],
                [lerp(matrix[2][0], my_m[2][0], percent),  lerp(matrix[2][1], my_m[2][1], percent)]
            ])

        } else {
            self.matrix
        }
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
        self.shared2.cursor_hovered_widget.get()
    }

    /// Signals the context that the cursor is currently hovering it. This can be later retreived
    /// with `cursor_hovered_widget()`.
    #[inline]
    pub fn set_cursor_hovered_widget(&self) {
        self.shared1.cursor_hovered_widget.store(true, Ordering::Relaxed);
        self.shared2.cursor_hovered_widget.set(true);
    }

    /// Reserves a new ID for a widget. Calling this function multiple times always returns
    /// a different id.
    #[inline]
    pub fn reserve_widget_id(&self) -> WidgetId {
        self.shared1.next_widget_id.fetch_add(1, Ordering::Relaxed).into()
    }

    /// Returns true if the cursor is currently hovering this part of the viewport.
    ///
    /// This is equivalent to `cursor_hover_coordinates().is_some()`, except more optimized.
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
            test(&self.matrix(), &cursor)
        } else {
            false
        }
    }

    /// If the cursor is hovering the context, returns the coordinates of the cursor within the
    /// context.
    ///
    /// The result is in OpenGL-like coordinates. In other words, (-1,-1) is the bottom-left hand
    /// corner and (1,1) is the top-right hand corner.
    pub fn cursor_hover_coordinates(&self) -> Option<[f32; 2]> {
        // we compute the inverse of the matrix
        let m = match self.matrix().invert() {
            Some(m) => m,
            None => return None,
        };

        // and use it to calculate the position of the cursor within the current context
        let in_pos = match self.cursor {
            Some(p) => p,
            None => return None,
        };

        let output_mouse = [
            in_pos[0]*m[0][0] + in_pos[1]*m[1][0] + m[2][0],
            in_pos[0]*m[0][1] + in_pos[1]*m[1][1] + m[2][1],
            in_pos[0]*m[0][2] + in_pos[1]*m[1][2] + m[2][2],
        ];

        let output_mouse = [output_mouse[0] / output_mouse[2], output_mouse[1] / output_mouse[2]];

        if output_mouse[0] < -1.0 || output_mouse[0] > 1.0 || output_mouse[0] != output_mouse[0] ||
           output_mouse[1] < -1.0 || output_mouse[1] > 1.0 || output_mouse[1] != output_mouse[1]
        {
            return None;
        }

        Some(output_mouse)
    }

    /// Returns the ratio of the width of the surface divided by its height.
    #[inline]
    pub fn width_per_height(&self) -> f32 {
        self.width / self.height
    }

    /// Builds a new draw context containing a subarea of the current context, but with a margin.
    ///
    /// The margin is expressed in percentage of the surface (between 0.0 and 1.0).
    #[inline]
    pub fn margin(&self, top: f32, right: f32, bottom: f32, left: f32) -> DrawContext<'b, D> {
        DrawContext {
            matrix: self.matrix * Matrix::translate(left - right, bottom - top)
                                * Matrix::scale_wh(1.0 - right - left, 1.0 - top - bottom),
            width: self.width * (1.0 - left - right),
            height: self.height * (1.0 - top - bottom),
            shared1: self.shared1.clone(),
            shared2: self.shared2.clone(),
            animation: self.animation,
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }

    /// Builds a new draw context containing a subarea of the current context, but with a margin.
    ///
    /// If the width of the surface is inferior to the height then the margin is expressed as a
    /// percentage of the width, and vice versa.
    ///
    /// This guarantees that the size in pixels of the margin is the same if you pass the same
    /// values.
    #[inline]
    pub fn uniform_margin(&self, top: f32, right: f32, bottom: f32, left: f32)
                          -> DrawContext<'b, D>
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
                                          -> DrawContext<'b, D>
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
                                        -> DrawContext<'b, D>
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

    /// Builds a new draw context containing a subarea of the current context. The width of the new
    /// viewport will be the same as the current one, but its new height will be multipled by
    /// the value of `scale`.
    ///
    /// The alignment is used to determine the position of the new viewport inside the old one.
    #[inline]
    pub fn vertical_rescale(&self, scale: f32, alignment: &VerticalAlignment)
                            -> DrawContext<'b, D>
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
            animation: self.animation,
            shared1: self.shared1.clone(),
            shared2: self.shared2.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }

    /// Builds a new draw context containing a subarea of the current context. The height of the new
    /// viewport will be the same as the current one, but its new width will be multipled by
    /// the value of `scale`.
    ///
    /// The alignment is used to determine the position of the new viewport inside the old one.
    #[inline]
    pub fn horizontal_rescale(&self, scale: f32, alignment: &HorizontalAlignment)
                              -> DrawContext<'b, D>
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
            animation: self.animation,
            shared1: self.shared1.clone(),
            shared2: self.shared2.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }

    /// Splits the viewport in `splits` vertical chunks of equal size.
    #[inline]
    pub fn vertical_split<'a>(&'a self, splits: usize) -> SplitsIter<'a, 'b, OneGen, D> {
        let iter = OneGen { n: splits };
        self.vertical_split_weights(iter)
    }

    /// Same as `vertical_split`, but attributes a weight to each chunk. For example a chunk of
    /// weight 2 will have twice the size of a chunk of weight 1.
    #[inline]
    pub fn vertical_split_weights<'a, I>(&'a self, weights: I) -> SplitsIter<'a, 'b, I::IntoIter, D>
        where I: IntoIterator<Item = f32>, I::IntoIter: ExactSizeIterator + Clone
    {
        self.split_weights(weights.into_iter(), true)
    }

    /// Splits the viewport in `splits` horizontal chunks of equal size.
    #[inline]
    pub fn horizontal_split<'a>(&'a self, splits: usize) -> SplitsIter<'a, 'b, OneGen, D> {
        let iter = OneGen { n: splits };
        self.horizontal_split_weights(iter)
    }

    /// Same as `horizontal_split`, but attributes a weight to each chunk. For example a chunk of
    /// weight 2 will have twice the size of a chunk of weight 1.
    #[inline]
    pub fn horizontal_split_weights<'a, I>(&'a self, weights: I) -> SplitsIter<'a, 'b, I::IntoIter, D>
        where I: IntoIterator<Item = f32>, I::IntoIter: ExactSizeIterator + Clone
    {
        self.split_weights(weights.into_iter(), false)
    }

    /// Internal implementation of the split functions.
    #[inline]
    fn split_weights<'a, I>(&'a self, weights: I, vertical: bool) -> SplitsIter<'a, 'b, I, D>
        where I: ExactSizeIterator<Item = f32> + Clone
    {
        assert!(weights.len() != 0);

        let total_weight = weights.clone().fold(0.0, |a, b| a + b);
        let total_weight_inverse = 1.0 / total_weight;

        SplitsIter {
            parent: self,
            weights: weights,
            total_weight_inverse: total_weight_inverse,
            current_offset: 0.0,
            vertical: vertical,
        }
    }

    /// Changes the dimensions of the context.
    ///
    /// The dimensions are a percentage of the current dimensions. For example to divide the width
    /// by two, you need to pass `0.5`.
    ///
    /// The alignment is used to determine the position of the newly-created context relative to
    /// the old one.
    pub fn rescale(&self, width_percent: f32, height_percent: f32, alignment: &Alignment)
                   -> DrawContext<'b, D>
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
            animation: self.animation,
            shared1: self.shared1.clone(),
            shared2: self.shared2.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }

    /// Starts an animation. The interpolation, start time and duration are used to calculate
    /// at which point of the animation we are.
    ///
    /// At the moment where you call this function, the element must be at its starting point.
    /// After you call this function, you must add further transformations to represent the
    /// destination. Don't forget to call `animation_stop` if you want to add further
    /// transformations.
    ///
    /// You can easily reverse this order (ie. the element is at its destination when you call the
    /// function and will be moved to its source) by reversing the interpolation with `.reverse()`.
    #[inline]
    pub fn animation_start<I>(&self, interpolation: I, start_time: SystemTime, duration: Duration)
                              -> DrawContext<'b, D>
        where I: Interpolation
    {
        let now = SystemTime::now();

        let interpolation = interpolation.calculate(now, start_time, duration) as f32;
        let current_matrix = self.matrix();

        DrawContext {
            matrix: self.matrix,
            width: self.width,
            height: self.height,
            animation: Some((current_matrix, interpolation)),
            shared1: self.shared1.clone(),
            shared2: self.shared2.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }

    /// Stops the animation process. The next commands will always be applied.
    #[inline]
    pub fn animation_stop(&self) -> DrawContext<'b, D> {
        DrawContext {
            matrix: self.matrix(),
            width: self.width,
            height: self.height,
            animation: None,
            shared1: self.shared1.clone(),
            shared2: self.shared2.clone(),
            cursor: self.cursor,
            cursor_was_pressed: self.cursor_was_pressed,
            cursor_was_released: self.cursor_was_released,
        }
    }
}

impl<'a, 'b, D: ?Sized + Draw + 'b> Clone for DrawContext<'b, D> {
    fn clone(&self) -> DrawContext<'b, D> {
        DrawContext {
            matrix: self.matrix.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            animation: self.animation.clone(),
            shared1: self.shared1.clone(),
            shared2: self.shared2.clone(),
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

/// Iterator that splits a context in pieces and returns new contexts.
pub struct SplitsIter<'a, 'b: 'a, I, D: ?Sized + Draw + 'b> {
    parent: &'a DrawContext<'b, D>,
    weights: I,
    total_weight_inverse: f32,
    current_offset: f32,
    vertical: bool,
}

impl<'a, 'b: 'a, I, D: ?Sized + Draw + 'b> Iterator for SplitsIter<'a, 'b, I, D>
    where I: Iterator<Item = f32>
{
    type Item = DrawContext<'b, D>;

    fn next(&mut self) -> Option<Self::Item> {
        let weight = match self.weights.next() {
            Some(w) => w,
            None => return None
        };

        let new_width = if !self.vertical { self.parent.width * weight * self.total_weight_inverse }
                        else { self.parent.width };
        let new_height = if self.vertical { self.parent.height * weight * self.total_weight_inverse }
                         else { self.parent.height };

        let scale_matrix = if self.vertical {
            Matrix::scale_wh(1.0, weight * self.total_weight_inverse)
        } else {
            Matrix::scale_wh(weight * self.total_weight_inverse, 1.0)
        };

        let pos_matrix = if self.vertical {
            let y = 1.0 - 2.0 * (self.current_offset + weight * 0.5) * self.total_weight_inverse;
            Matrix::translate(0.0, y)
        } else {
            let x = 2.0 * (self.current_offset + weight * 0.5) * self.total_weight_inverse - 1.0;
            Matrix::translate(x, 0.0)
        };

        self.current_offset += weight;

        Some(DrawContext {
            matrix: self.parent.matrix * pos_matrix * scale_matrix,
            width: new_width,
            height: new_height,
            animation: self.parent.animation,
            shared1: self.parent.shared1.clone(),
            shared2: self.parent.shared2.clone(),
            cursor: self.parent.cursor,
            cursor_was_pressed: self.parent.cursor_was_pressed,
            cursor_was_released: self.parent.cursor_was_released,
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.weights.size_hint()
    }
}

impl<'a, 'b: 'a, I, D: ?Sized + Draw + 'b> ExactSizeIterator for SplitsIter<'a, 'b, I, D>
    where I: ExactSizeIterator<Item = f32>
{
}

/// Iterator that generates `1.0` a certain number of times.
// TODO: This is required so that `horizontal_split` and `vertical_split` can express their
//       return type. Should be replaced with `-> impl Iterator` eventually.
#[derive(Debug, Clone)]
pub struct OneGen {
    n: usize
}

impl Iterator for OneGen {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        if self.n == 0 { return None; }
        self.n -= 1;
        Some(1.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.n, Some(self.n))
    }
}

impl ExactSizeIterator for OneGen {
}

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use time;

use Draw;
use Matrix;

use animations::Animation;

/// Contains everything required to draw a widget.
pub struct DrawContext<'a, D: ?Sized + Draw + 'a> {
    matrix: Matrix,
    width: f32,
    height: f32,
    draw: Arc<Mutex<&'a mut D>>,

    cursor: Option<[f32; 2]>,
}

impl<'a, D: ?Sized + Draw + 'a> DrawContext<'a, D> {
    // TODO: change this
    /// UNSTABLE, WILL BE CHANGED
    pub fn start(width: f32, height: f32, draw: &'a mut D, cursor: Option<[f32; 2]>) -> DrawContext<'a, D> {
        DrawContext {
            matrix: Matrix::identity(),
            width: width,
            height: height,
            draw: Arc::new(Mutex::new(draw)),
            cursor: cursor,
        }
    }

    /// UNSTABLE. Obtains the underlying `draw` object.
    #[inline]
    pub fn draw(&self) -> MutexGuard<&'a mut D> {
        self.draw.lock().unwrap()
    }

    #[inline]
    pub fn matrix(&self) -> &Matrix {
        &self.matrix
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
    /// The margin is expressed in percentage of the surface.
    #[inline]
    pub fn margin(&self, top: f32, right: f32, bottom: f32, left: f32) -> DrawContext<'a, D> {
        // TODO: could be more efficient
        self.absolute(1.0 - left, 1.0 - top, &Alignment::center())
            .absolute(1.0 - right, 1.0 - bottom, &Alignment::center())
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
            draw: self.draw.clone(),
            cursor: self.cursor,
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
            draw: self.draw.clone(),
            cursor: self.cursor,
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
                draw: self.draw.clone(),
                cursor: self.cursor,
            }
        }).collect()
    }

    pub fn absolute(&self, width_percent: f32, height_percent: f32, alignment: &Alignment)
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
            draw: self.draw.clone(),
            cursor: self.cursor,
        }
    }

    pub fn animate<A>(&self, animation: A, start_time: u64, duration_ns: u64,
                      initial_pos: [f32; 2]) -> DrawContext<'a, D> where A: Animation
    {
        let now = time::precise_time_ns();

        let x = animation.calculate(now, start_time, duration_ns, initial_pos[0]);
        let y = animation.calculate(now, start_time, duration_ns, initial_pos[1]);

        DrawContext {
            matrix: self.matrix * Matrix::translate(x, y),
            width: self.width,
            height: self.height,
            draw: self.draw.clone(),
            cursor: self.cursor,
        }
    }
}

impl<'a, D: ?Sized + Draw + 'a> Clone for DrawContext<'a, D> {
    fn clone(&self) -> DrawContext<'a, D> {
        DrawContext {
            matrix: self.matrix.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            draw: self.draw.clone(),
            cursor: self.cursor.clone(),
        }
    }
}

/// Represents the alignment of a viewport.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Alignment {
    pub horizontal: HorizontalAlignment,
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HorizontalAlignment {
    Center,
    Left,
    Right,
}

impl Default for HorizontalAlignment {
    #[inline]
    fn default() -> HorizontalAlignment {
        HorizontalAlignment::Center
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VerticalAlignment {
    Center,
    Top,
    Bottom,
}

impl Default for VerticalAlignment {
    #[inline]
    fn default() -> VerticalAlignment {
        VerticalAlignment::Center
    }
}

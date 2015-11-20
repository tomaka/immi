//! Contains all animations interpolations that are supported by this library.
//!
//! The `Animation` trait is the trait that is implemented for all other structs of this module.

///
pub trait Animation {
    /// Takes a number of ticks (in nanoseconds) representing the current point in time, a number
    /// of ticks representing the point in time when the animation has started or will start, the
    /// duration in nanoseconds, and returns a value between 0.0 and 1.0 representing the progress
    /// of the animation.
    ///
    /// Implementations typically return `0.0` when `now < start` and `1.0` when
    /// `now > start + duration_ns`.
    fn calculate(&self, now: u64, start: u64, duration_ns: u64) -> f32;
}

/// An ease-out animation. The element moves quickly and then slows down before reaching its
/// final position.
pub struct EaseOut {
    /// The formula is `1.0 - exp(-linear_progress * factor)`.
    ///
    /// The higher the factor, the quicker the element will reach its destination.
    pub factor: f32,
}

impl Default for EaseOut {
    #[inline]
    fn default() -> EaseOut {
        EaseOut { factor: 10.0 }
    }
}

impl Animation for EaseOut {
    #[inline]
    fn calculate(&self, now: u64, start: u64, duration_ns: u64) -> f32 {
        if now < start {
            return 0.0;
        }

        let anim_progress = (now - start) as f32 / duration_ns as f32;
        1.0 - (-anim_progress * self.factor).exp()
    }
}

//! Contains everything related to the animations that are supported by this library.

use std::time::Duration;
use std::time::SystemTime;

/// Describes how an animation should be interpolated.
pub trait Interpolation {
    /// Takes an instance representing the current point in time, an instant representing the
    /// point in time when the animation has started or will start, the duration, and returns a
    /// value between 0.0 and 1.0 representing the progress of the animation.
    ///
    /// Implementations typically return `0.0` when `now < start` and `1.0` when
    /// `now > start + duration_ns`.
    fn calculate(&self, now: SystemTime, start: SystemTime, duration: Duration) -> f32;

    /// Reverses and interpolation. The element will start at its final position and go towards
    /// the start.
    #[inline]
    fn reverse(self) -> Reversed<Self> where Self: Sized {
        Reversed::new(self)
    }
}

/// A linear animation. The animation progresses at a constant rate.
#[derive(Copy, Clone, Default, Debug)]
pub struct Linear;

impl Interpolation for Linear {
    #[inline]
    fn calculate(&self, now: SystemTime, start: SystemTime, duration: Duration) -> f32 {
        let now_minus_start_ms = {
            let v = now.duration_since(start).unwrap_or(Duration::new(0, 0));
            v.as_secs() as f64 * 1000000.0 + v.subsec_nanos() as f64 / 1000.0
        };

        let duration_ms = duration.as_secs() as f64 * 1000000.0 +
                          duration.subsec_nanos() as f64 / 1000.0;

        let anim_progress = (now_minus_start_ms / duration_ms) as f32;
        
        if anim_progress >= 1.0 {
            1.0
        } else if anim_progress <= 0.0 {
            0.0
        } else {
            anim_progress
        }
    }
}

/// An ease-out animation. The animation progresses quickly and then slows down before reaching its
/// final position.
#[derive(Copy, Clone, Debug)]
pub struct EaseOut {
    /// The formula is `1.0 - exp(-linear_progress * factor)`.
    ///
    /// The higher the factor, the quicker the element will reach its destination.
    pub factor: f32,
}

impl EaseOut {
    /// Builds a `EaseOut` object.
    #[inline]
    pub fn new(factor: f32) -> EaseOut {
        EaseOut {
            factor: factor,
        }
    }
}

impl Default for EaseOut {
    #[inline]
    fn default() -> EaseOut {
        EaseOut { factor: 10.0 }
    }
}

impl Interpolation for EaseOut {
    #[inline]
    fn calculate(&self, now: SystemTime, start: SystemTime, duration: Duration) -> f32 {
        let now_minus_start_ms = {
            let v = match now.duration_since(start) {
                Ok(v) => v,
                Err(_) => return 0.0,
            };

            v.as_secs() as f64 * 1000000.0 + v.subsec_nanos() as f64 / 1000.0
        };

        let duration_ms = duration.as_secs() as f64 * 1000000.0 +
                          duration.subsec_nanos() as f64 / 1000.0;

        let anim_progress = (now_minus_start_ms / duration_ms) as f32;
        1.0 - (-anim_progress * self.factor).exp()
    }
}

/// Wraps around an interpolation and reverses it. The element will start at its final position
/// and go towards the start.
#[derive(Copy, Clone, Debug)]
pub struct Reversed<I> {
    inner: I
}

impl<I> Reversed<I> where I: Interpolation {
    /// Builds a `Reversed` object.
    #[inline]
    pub fn new(inner: I) -> Reversed<I> {
        Reversed {
            inner: inner,
        }
    }
}

impl<I> Interpolation for Reversed<I> where I: Interpolation {
    #[inline]
    fn calculate(&self, now: SystemTime, start: SystemTime, duration: Duration) -> f32 {
        1.0 - self.inner.calculate(now, start, duration)
    }
}

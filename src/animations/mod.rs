
pub trait Animation {
    fn calculate(&self, now: u64, start: u64, duration_ns: u64, initial_value: f32) -> f32;
}

pub struct EaseOut {
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
    fn calculate(&self, now: u64, start: u64, duration_ns: u64, initial_value: f32) -> f32 {
        let anim_progress = (now - start) as f32 / duration_ns as f32;
        (-anim_progress * self.factor).exp() * initial_value
    }
}

pub struct D1;

impl D1 {
    pub fn clamp(value : f32, min : f32, max : f32) -> f32 {
        if value < min.min(max) {
            min.min(max)
        } else if value > min.max(max) {
            min.max(max)
        } else {
            value
        }
    }

    pub fn clamp01(value : f32) -> f32 {
        Self::clamp(value, 0.0, 1.0)
    }

    pub fn normalize_from_to(value : f32, from_min : f32, from_max : f32, to_min : f32, to_max : f32) -> f32 {
        (to_min.max(to_max) - to_min.min(to_max))
        * (value - from_min.min(from_max))
        / (from_min.max(from_max) - from_min.min(from_max))
        + to_min.min(to_max)
    }

    pub fn normalize_from_01(value : f32, min : f32, max : f32) -> f32 {
        value * (min.max(max) - min.min(max)) + min
    }

    pub fn normalize_to_01(value : f32, min : f32, max : f32) -> f32 {
        (value - min.min(max))/(min.max(max) - min.min(max))
    }
}
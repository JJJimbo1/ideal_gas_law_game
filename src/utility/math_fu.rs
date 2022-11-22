use std::f32::consts::PI;

pub struct D1;

impl D1 {
    // pub fn approx<F : Float + FromPrimitive>(value : F, value2 : F) -> bool {
    //     (value - value2).abs() < FromPrimitive::from_f32(0.0001).unwrap()
    // }

    // pub fn approx_dif<F : Float + FromPrimitive>(value : F, value2 : F, dif : F) -> bool {
    //     (value - value2).abs() < dif
    // }

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

    // pub fn farther_from_zero<F : Float>(value : F, base : F) -> bool {
    //     value.abs() > base.abs()
    // }

    // pub fn lerp<F : Float + FromPrimitive>(value : F, target : F, ramp : F) -> F {
    //     value + (target - value) * Self::clamp01(ramp)
    // }

    // pub fn more_than_or_zero<F : Float + FromPrimitive>(value : F) -> F {
    //     if value > FromPrimitive::from_f32(0.0001).unwrap() {
    //         value
    //     } else {
    //         FromPrimitive::from_f32(0.0).unwrap()
    //     }
    // }

    // pub fn more_than_value_or_value<F : Float>(value : F, min : F, default : F) -> F {
    //     if value > min {
    //         value
    //     } else {
    //         default
    //     }
    // }

    // pub fn more_than_value_or_zero<F : Float + FromPrimitive>(value : F, min : F) -> F {
    //     if value > min {
    //         value
    //     } else {
    //         FromPrimitive::from_f32(0.0).unwrap()
    //     }
    // }

    // pub fn more_than_or_zero_pog<F : Float + FromPrimitive>(value : F) -> F {
    //     if value.abs() > FromPrimitive::from_f32(0.0001).unwrap() {
    //         value
    //     } else {
    //         FromPrimitive::from_f32(0.0).unwrap()
    //     }
    // }

    // pub fn more_than_value_or_value_pog<F : Float>(value : F, min : F, default : F) -> F {
    //     if value.abs() > min {
    //         value
    //     } else {
    //         default
    //     }
    // }

    // pub fn more_than_value_or_zero_pog<F : Float + FromPrimitive>(value : F, min : F) -> F {
    //     if value.abs() > min {
    //         value
    //     } else {
    //         FromPrimitive::from_f32(0.0).unwrap()
    //     }
    // }

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

    // ø = ∛(6 · V / π)

    pub fn diameter_from_volume(volume : f32) -> f32 {
        (6.0 * volume / PI).cbrt()
    }

    pub fn volume_from_diameter(diameter : f32) -> f32 {
        (1.0 / 6.0) * PI * diameter.powf(3.0)
    }

    // pub fn powf_sign<F : Float + FromPrimitive>(value : F, pow : F) -> F {
    //     if value > FromPrimitive::from_f32(0.0).unwrap() {
    //         value.abs().powf(pow)
    //     } else if value < FromPrimitive::from_f32(0.0).unwrap() {
    //         -(value.abs().powf(pow))
    //     } else {
    //         value
    //     }
    // }

    // pub fn powi_sign<F : Float + FromPrimitive>(value : F, pow : i32) -> F {
    //     if value > FromPrimitive::from_f32(0.0).unwrap() {
    //         value.powi(pow)
    //     } else if value < FromPrimitive::from_f32(0.0).unwrap() {
    //         -(value.abs().powi(pow))
    //     } else {
    //         value
    //     }
    // }

    // pub fn same_sign<F : Float>(value : F, base : F) -> bool {
    //     value.signum() == base.signum()
    // }
}
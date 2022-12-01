pub mod sphere {
    use std::f32::consts::PI;

    pub fn volume_from_diameter(diameter : f32) -> f32 {
        (1.0 / 6.0) * PI * diameter.powf(3.0)
    }

    pub fn diameter_from_volume(volume : f32) -> f32 {
        (6.0 * volume / PI).cbrt()
    }

}

pub mod cylinder {
    use std::f32::consts::PI;
    
    pub fn liters_from_radius_and_height(radius: f32, height: f32) -> f32 {
        PI * radius.powf(2.0) * height * 1000.0
    }
}
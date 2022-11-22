use bevy::prelude::Resource;

use crate::utility::math_fu;



pub const GAS_CONSTANT: f32 = 8.31446261815324;
pub const STANDARD_TEMPERATURE: f32 = 273.15;
pub const STANDARD_PRESSURE: f32 = 101_325.0;


#[derive(Debug, Clone, Copy)]
#[derive(Resource)]
pub struct GasContainer {
    pub pressure: f32, // Pa
    pub cubic_meters: f32, //liters
    pub max_cubic_meters: f32,
    pub temperature: f32, //kelvin
    pub moles: f32,
}

impl GasContainer {
    pub fn new_moles(moles: f32) -> Self {
        let pressure = STANDARD_PRESSURE;
        let temperature = STANDARD_PRESSURE;
        let cubic_meters = (moles * temperature * GAS_CONSTANT) / pressure;

        Self {
            pressure,
            cubic_meters,
            max_cubic_meters: 4.19,
            temperature,
            moles,
        }
    }

    pub fn new_volume(cubic_meters: f32) -> Self {
        let pressure = 101_325.0;
        let temperature = 273.15;
        let moles = (pressure * cubic_meters) / (temperature * GAS_CONSTANT);
        println!("{}", moles);

        Self {
            pressure,
            cubic_meters,
            max_cubic_meters: 4.19,
            temperature,
            moles,
        }
    }

    pub fn calculate_pressure(&mut self) -> f32 {
        let pressure = (self.moles * self.temperature * GAS_CONSTANT) / self.cubic_meters;
        self.pressure = pressure;
        pressure
    }

    pub fn calculate_volume(&mut self) -> f32 {
        let cubic_meters = (self.moles * self.temperature * GAS_CONSTANT) / self.pressure;
        self.cubic_meters = cubic_meters;
        cubic_meters
    }

    // pub fn calculate_moles(&self) -> f32 {
    //     5.0
    // }

    // ///Returns an f32 between 0 and 1 depending on pressure.
    // pub fn progress(&self) -> f32 {
    //     math_fu::D1::normalize_to_01(self.pressure, STANDARD_PRESSURE, self.max_pressure).clamp(0.0, 1.0)
    // }

}
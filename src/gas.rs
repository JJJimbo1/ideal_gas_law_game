use bevy::prelude::Resource;


pub const GAS_CONSTANT: f32 = 8.31446261815324;
pub const STANDARD_TEMPERATURE: f32 = 273.0;
pub const STANDARD_PRESSURE: f32 = 101_325.0;


#[derive(Debug, Clone, Copy)]
#[derive(Resource)]
pub struct GasContainer {
    pub pressure: f32, // Pa
    pub cubic_meters: f32, //liters
    pub temperature: f32, //kelvin
    pub moles: f32,
}

impl GasContainer {
    pub fn new_moles(moles: f32) -> Self {
        Self {
            pressure: STANDARD_PRESSURE,
            cubic_meters: (moles * STANDARD_TEMPERATURE * GAS_CONSTANT) / STANDARD_PRESSURE,
            temperature: STANDARD_TEMPERATURE,
            moles,
        }
    }

    pub fn new_volume(cubic_meters: f32) -> Self {
        Self {
            pressure: STANDARD_PRESSURE,
            cubic_meters,
            temperature: STANDARD_TEMPERATURE,
            moles: (STANDARD_PRESSURE * cubic_meters) / (STANDARD_TEMPERATURE * GAS_CONSTANT),
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

    pub fn calculate_temperature(&mut self) -> f32 {
        let temperature = (self.pressure * self.cubic_meters) / (self.moles * GAS_CONSTANT);
        self.temperature = temperature;
        temperature
    }

    pub fn calculate_moles(&mut self) -> f32 {
        (self.pressure * self.cubic_meters) / (self.temperature * GAS_CONSTANT)
    }
}
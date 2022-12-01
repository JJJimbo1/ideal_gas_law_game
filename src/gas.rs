use bevy::prelude::Resource;


pub const GAS_CONSTANT: f32 = 0.082057;
pub const STANDARD_TEMPERATURE: f32 = 273.0;
pub const STANDARD_PRESSURE: f32 = 1.0;


#[derive(Debug, Clone, Copy)]
#[derive(Resource)]
pub struct GasContainer {
    pub atm: f32,
    pub liters: f32,
    pub kelvin: f32,
    pub moles: f32,
}

impl GasContainer {
    pub fn new_moles(moles: f32) -> Self {
        Self {
            atm: STANDARD_PRESSURE,
            liters: (moles * STANDARD_TEMPERATURE * GAS_CONSTANT) / STANDARD_PRESSURE,
            kelvin: STANDARD_TEMPERATURE,
            moles,
        }
    }

    pub fn new_volume(liters: f32) -> Self {
        Self {
            atm: STANDARD_PRESSURE,
            liters,
            kelvin: STANDARD_TEMPERATURE,
            moles: (STANDARD_PRESSURE * liters) / (STANDARD_TEMPERATURE * GAS_CONSTANT),
        }
    }

    pub fn calculate_pressure(&mut self) -> f32 {
        let atm = (self.moles * self.kelvin * GAS_CONSTANT) / self.liters;
        self.atm = atm;
        atm
    }

    pub fn calculate_volume(&mut self) -> f32 {
        let liters = (self.moles * self.kelvin * GAS_CONSTANT) / self.atm;
        self.liters = liters;
        liters
    }

    pub fn calculate_temperature(&mut self) -> f32 {
        let kelvin = (self.atm * self.liters) / (self.moles * GAS_CONSTANT);
        self.kelvin = kelvin;
        kelvin
    }

    pub fn calculate_moles(&mut self) -> f32 {
        (self.atm * self.liters) / (self.kelvin * GAS_CONSTANT)
    }
}
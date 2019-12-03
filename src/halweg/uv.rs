
/// Furnace

use embedded_hal::digital::v2::{OutputPin,InputPin};
// use embedded_hal::timer::CountDown;
// use embedded_hal::timer::Periodic;
// use nb::block;
// use super::error::Error;
// use super::pump::Pump;
// use super::valve::Valve;

pub struct NDir1;
pub struct NDir2;
pub struct GP1;

pub struct Y1;
pub struct Y9;

pub struct CH1;
pub struct CH2;
pub struct CH3;
pub struct CH4;
pub struct CH5;


/// Beeper
pub struct Uv
{
    pub gp: GP1,
    pub sampl: Y1,
    pub cal: Y9,
    pub ndir1: NDir1,
    pub ndir2: NDir2,
    pub ch1:   CH1,
    pub ch2:   CH2,
    pub ch3:   CH3,
    pub ch4:   CH4,
    pub ch5:   CH5,
    // rt:  u64,
    // st:  u64,
}

// impl<PIN,ISN> Uv<PIN,ISN>
// where
    // PIN: OutputPin,
    // ISN: InputPin,
// {
    // pub fn create(gp: Pump<PIN>) -> Self {
        // Uv { gp }
    // }

// }


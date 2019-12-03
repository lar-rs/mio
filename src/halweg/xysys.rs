
/// Autosampler
///
///
///
use nb;

pub trait Axis {
    type Position;
    type Error;
    type Velocity;

    fn position(&self) -> nb::Result<Self::Position,Self::Error>;
    fn move(&mut self,Self::Position) -> nb::Result<(),Self::Error>;
    fn is_sensor(&mut self) -> nb::Result<bool,Self::Error>;
    fn x_to_sensor(&mut self) -> nb::Result<bool,Self::Error>;
}


pub trait XYSystem{
    type Error;
    type Position;
    type Velocity;
    fn xposition(&mut self) -> nb::Result<Self::Position,Self::Error>;
    fn yposition(&mut self) -> nb::Result<Self::Position,Self::Error>;
    fn zposition(&mut self) -> nb::Result<Self::Position,Self::Error>;

    fn xhold(&mut self) -> nb::Result<Self::Position,Self::Error>;
    fn yhold(&mut self) -> nb::Result<Self::Position,Self::Error>;
    fn zhold(&mut self) -> nb::Result<Self::Position,Self::Error>;

    fn x_to_sensor(&mut self) -> nb::Result<(),Self::Error>;
    fn y_to_sensor(&mut self) -> nb::Result<(),Self::Error>;
    fn z_to_sensor(&mut self) -> nb::Result<(),Self::Error>;
    fn xmove(&mut self,par:Self::Velocity, pos:Self::Position) -> nb::Result<(),Self::Error>;
    fn ymove(&mut self,par:Self::Velocity, pos:Self::Position) -> nb::Result<(),Self::Error>;
    fn zmove(&mut self,par:Self::Velocity, pos:Self::Position) -> nb::Result<(),Self::Error>;

    fn take(&mut self, par:Self::Velocity) -> nb::Result<(),Self::Error>;
    fn push(&mut self, par:Self::Velocity) -> nb::Result<(),Self::Error>;
    fn rinse(&mut self) -> nb::Result<(),Self::Error>;
    fn injection(&mut self) -> nb::Result<(),Self::Error>;
}


#[cfg(feature = "mosk")]
pub mod mosk {
    use crate::error::MockError;
    use crate::common::Generic;


// /// Analog Out mock
// ///
// /// Models an Analog read or write
// #[derive(Clone, Debug, PartialEq)]
// pub struct MoskAnalogOut{
//     value: f32;
// }


// impl Analog for MoskAnalogOut {
//     type Error = MockError;
//     type Value = f32;

//     fn set_value (&mut self, v : Value) ->Result<(), Self::Error> {
//         self.value = v;
//         Ok(())
//     }
}



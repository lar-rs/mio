/// Monitor gear pump normally used for solution sampling.
///
///


pub trait Pump {

    type Error;
    fn start ( &mut self)  -> nb::Result<(), Self::Error>;
    fn stop( &mut self) -> nb::Result<(), Self::Error>;
}





#[cfg(feature = "mosk")]
pub mod mosk {
    use crate::error::MockError;
    use crate::common::Generic;


// Analog Out mock
//
// #[derive(Clone, Debug, PartialEq)]
// pub struct MoskAnalogOut{
    // value: f32;
// }


// impl Analog for MoskAnalogOut {
    // type Error = MockError;
    // type Value = f32;

    // fn set_value (&mut self, v : Value) ->Result<(), Self::Error> {
        // self.value = v;
        // Ok(())
    // }
}

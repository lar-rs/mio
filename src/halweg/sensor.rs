/// Sensor HAL
/// NDir1,NDir2 Sauerstoff
///
///



pub trait Sensor {
    type Error;

    fn signal(&mut self) -> nb::Result<f32, Self::Error>;
}




//! Valve

pub enum State {
    Close,
    Open,
}

pub struct Valve
{
    state: State,
}
/// Single digital push-pull output pin
///
impl Valve
{
    /// Valve open
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    pub fn set_open(&mut self) {
        self.state = State::Open;
    }
    /// Valve close
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    pub fn set_close(&mut self) {
        self.state = State::Open;
    }
}


pub type StreamSwitching;
pub type Calibration;
pub type DetectorBypass;
pub type NdirZeroCorrection;




pub async fn mesaurement1_way() -> Result<()>{
    Ok(())
}
pub async fn calibration1_way() -> Result<()>{
    Ok(())
}
pub async fn mesaurement2_way() -> Result<()>{
    Ok(())
}
pub async fn calibration2_way() -> Result<()>{
    Ok(())
}
///

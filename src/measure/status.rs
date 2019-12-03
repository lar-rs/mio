use serde::{Deserialize, Serialize};


pub enum ProcessState {
    Wait,
    Measurement,
    Calibrate,
    Check,
}



pub enum MeasurementStreams {
    Stream1State{
        pub measurement:bool,
    },
    MeasurementStream2(StreamState),
    MeasurementStream3,
    MeasurementStream4,
    MeasurementStream5,
    MeasurementStream6,
}

pub struct StreamState{
    pub process: ProcessState,

}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Status{
    pub online:bool,
    pub critical:bool,
    pub warnung:bool,
    pub errors:u32,
    pub calibration: 
}
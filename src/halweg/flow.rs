/// Flow monitoring
/// NDir1,NDir2 Sauerstoff
///
///

use serde::{Deserialize, Serialize};
// use core::ops::Range;
use super::{
    Airflow,
    Humidity,
    Pressure,
};
// pub type FlowRange = Range<f32>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MonitoringSetting {
    pub humidity_crit_level:  f32,
    pub pressure_warn_level:  f32,
    pub pressure_crit_level:  f32,
    pub airflow_correlation:  f32,
    pub airflow_soll_value:   f32,
    pub input_deviation:      f32,
    pub output_deviation:     f32,
    pub monitorin_interval:   u64,
    // pub injection_threshold:  f32,
}



impl Default for MonitoringSetting {
    fn default() -> Self {
        Self {
            humidity_crit_level:  70.0,
            pressure_warn_level:  300.0,
            pressure_crit_level:  600.0,
            airflow_correlation:  1.0,
            airflow_soll_value:   30.0,
            input_deviation:      0.0,
            output_deviation:     0.0,
            monitorin_interval:   0,
        }
    }
}



pub trait Monitoring {
    type Error;
    fn airflow_input(&self) -> Result<Airflow,Self::Error>;
    fn airflow_output(&self) -> Result<Airflow,Self::Error>;
    fn humidity(&self) -> Result<Humidity,Self::Error>;
    fn pressure(&self) -> Result<Pressure,Self::Error>;
}

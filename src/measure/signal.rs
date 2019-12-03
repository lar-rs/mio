use serde_derive::{Deserialize, Serialize};
use super::integration::Integration;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
/// start time in mili seconds
    pub start:   u64,
/// Pereodic 
    pub period:  u64,
/// Data  
    pub data: f64,
}


impl Default for Record {
    fn default() -> Self {
        Self {
            start:  0,
            period: 300,
            data:   0.0,
        }
    }
}

/// Average signal record;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Average {
/// start time in mili seconds
    pub start:   u64,
/// Pereodic 
    pub period:  u64,
/// Sensor data
    pub rec  :  Record,
}


impl Default for AverageRecord {
    fn default() -> Self {
        Self {
            start:  0,
            period: 300,
            data:   0.0,
        }
    }
}

/// Average signal record;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntegrationRecord {
/// start time in mili seconds
    pub start:   u64,
/// Pereodic 
    pub period:  u64,
/// Sensor data
    pub zero  :  Average,
/// Integration settings 
    pub integration: Integration,
/// data
}


impl Default for Average{
    fn default() -> Self {
        Self {
            start:  0,
            period: 300,
            data:   0.0,
        }
    }
}



pub enum Signal { 
    Average(Average),
    Integration(),
} 
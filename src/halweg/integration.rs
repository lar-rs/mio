//! Integration parameters
//! * Justification - time to emit last zero value.
//!
use serde::{Deserialize, Serialize};


/// Integration
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Integration {
    pub justification: u32,
    pub start_treshold: f64,
    pub stop_treshold: f64,
    pub min_start: u32,
    pub min_stop: u32,
    pub max_stop: u32,
}


impl Default for Integration {
    fn default() -> Self {
        Self {
            justification: 15,
            start_treshold: 0.002,
            stop_treshold: 0.003,
            min_start: 10,
            min_stop: 60,
            max_stop: 210,
        }
    }
}



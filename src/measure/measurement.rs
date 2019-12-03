use serde::{Deserialize, Serialize};
// use std::time::{Duration};

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct MeasReplicate {
//     pub value: f64,
//     pub outlier: bool,
//     pub signal: u64,
// //     // pub signal: Signal,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Measurement {
    pub value:      f64,
    pub timestamp:  u64,
}


impl Measurement {
    pub fn new() -> Measurement {
        Measurement{
            timestamp: 0,
            // updated: 0,
            // cv:      0.0,
            value:   0.0,
            // datas:    Vec::new(),
        }
    }
    // pub fn add(&mut self, data: MeasData ) {
        // self.updated = now_sec();
        // self.datas.push(data);
    // }
}

//

pub struct Measurements {
    
}
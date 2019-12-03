use serde_derive::{Deserialize, Serialize};
use nb;
use failure::Fail;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Speed {
    Slow,
    Normal,
}

#[derive(Fail, Debug)]
pub enum AxisError {
    #[fail(display = "Move to {} fail", _0)]
    Move(u32),
}
/// Any distance value (positive or negative)
pub type Distance = i32;

// #[derive(Clone,Copy, Debug, PartialEq)]

pub type Position = u32;




pub type Volume = u32;

// impl Volume {
//     fn air() -> Volume {
//         10 as Volume
//     }
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Injection {
    pub axis: AxisState,
    pub air: Volume,
    pub furnace_air: Volume,
    pub rinsing: Volume,
    pub dilution: Volume,
    pub rest : Volume,
    pub sampling: Volume
}

impl Default for Injection {
 fn default() ->Self {
       Self {
           axis: AxisState::default(),
           air: 10,
           furnace_air:50,
           rinsing: 450,
           dilution: 400,
           rest : 50,
           sampling:  150,
       }
   }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct XYSystem {
    x : AxisState,
    y : AxisState,
    inj: Injection,
    vessels:Vec<Vessel>,
}


impl Default for XYSystem {
 fn default() ->Self {
       Self {
           x: AxisState::default(),
           y: AxisState::default(),
           inj: Injection::default(),
           vessels: vec!(
               Vessel::default(),
               Vessel::default(),
               Vessel::default(),
               Vessel::default(),
               Vessel::default(),
               Vessel::default(),
               Vessel::default(),
               Vessel::default(),
           ),
       }
   }
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub enum Actions{
//     XMoveTo(Move),
//     YMoveTo(Move),
//     ZMoveTo(Move),
//     Vessel(Vessel),
//     State(XYSystem),
// }



pub struct XAxis {

}
impl XAxis {
    fn move_to(mut self, pos:Position) -> nb::Result<XAxis,AxisError> {
        Ok(self)
    }
}

pub struct YAxis {

}
pub struct InjAxis {

}


// impl XYZSystemState{
    // pub fn new() -> Self {
    //     Self {
    //         current:Flow::default(),
    //         calibration: Flow::default(),
    //         correction: 1.0,
    //         deviation: Deviation::default(),
    //         injection_deviation: Deviation::default(),
    //     }
    // }
// }

say_hi

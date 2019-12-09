use super::*;
use std::fs;
// use std::convert::{TryFrom};

// use std::collections::BTreeSet;
// use serde::{Serialize,Deserialize};

pub struct State {
    pub machine_id:      String,
    pub machine_info:    String,
    pub interfaces:      Vec<Interface>
}


pub fn get_current(ws:&Workspace) -> Result<State> {
    let machine_id = fs::read_to_string("/etc/machine-id")?;
    let machine_info = fs::read_to_string("/etc/machine-info")?;
    let interfaces = ws.mio.get_interfaces()?;
    Ok(State{ machine_id,machine_info,interfaces })
}
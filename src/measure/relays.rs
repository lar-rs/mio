/// Relay
///
use serde_derive::{Deserialize, Serialize};
use lazy_static::lazy_static;

lazy_static!{
      static ref RELAYS8: RwLock<Relays> = RwLock::new(Relays::new());
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Relay {
    pub hid: u64,
    pub open: bool,
}



impl Default for Relay {
    fn default() -> Self {
        Self{
            hid: 0,
            open : false,
        }
    }
}


/// Relay
impl Relay {
    /// new Relay
    pub fn new() -> Self {
        Self {
            open: true,

        }
    }
    /// Relay open
    ///
    pub fn open(&mut self) {
        self.open = false;
    }
    /// Relay close
    ///
    pub fn close(&mut self) {
        self.open = false;
    }
}



pub struct Relays {
    pub r1 : Relay,
    pub r2 : Relay,
    pub r3 : Relay,
    pub r4 : Relay,
    pub r5 : Relay,
    pub r6 : Relay,
    pub r7 : Relay,
    pub r8 : Relay,
}


impl Relays {
    pub fn new() -> Self {
        Self {
            r1 : Relay::new(),
            r2 : Relay::new(),
            r3 : Relay::new(),
            r4 : Relay::new(),
            r5 : Relay::new(),
            r6 : Relay::new(),
            r7 : Relay::new(),
            r8 : Relay::new(),
        }
    }
    pub fn status (&self) -> Vec<Relay> {
        vec![
            self.r1.clone(),
            self.r2.clone(),
            self.r3.clone(),
            self.r4.clone(),
            self.r5.clone(),
            self.r6.clone(),
            self.r7.clone(),
            self.r8.clone(),
        ]
    }
}

pub async fn status( )-> Vec<Relay> {
    RELAYS8.read().unwrap().status()
}

pub async fn open_relay() {

}

pub async fn close_relay() {

}



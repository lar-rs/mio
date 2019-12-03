use wqa::error::*;
use std::collections::HashMap;




use lazy_static::lazy_static;
// use std::sync::RwLock;
lazy_static! {
    static ref DOUT1: HashMap<u8,bool > = {
        let mut m = HashMap::new();
        for n in 0..16 {
            m.insert(n, false);
        }
       m
    };
    static ref DIN1: HashMap<u8,bool > = {
        let mut m = HashMap::new();
        for n in 0..16 {
            m.insert(n, false);
        }
       m
    };
    static ref DIN2: HashMap<u8,bool > = {
        let mut m = HashMap::new();
        for n in 0..16 {
            m.insert(n, false);
        }
       m
    };
    static ref DOUT2: HashMap<u8,bool > = {
        let mut m = HashMap::new();
        for n in 0..16 {
            m.insert(n, false);
        }
       m
    };
    static ref AIN: HashMap<u8,u16> = {
        let mut m = HashMap::new();
        m.insert(0,0);
        m.insert(1,0);
        m.insert(2,0);
        m.insert(3,0);
        m.insert(4,0);
        m
    }
    static ref AOUT: HashMap<u8,u16> = {
        let mut m = HashMap::new();
         for n in 0..16 {
            m.insert(n, false);
        }
        m.insert(0,0);
        m.insert(1,0);
        m.insert(2,0);
        m.insert(3,0);
        m.insert(4,0);
    }



}

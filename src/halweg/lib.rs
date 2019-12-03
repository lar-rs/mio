//! A LAR Hardware Abstraction Layer (HAL) anasync_traitd analytic model for water quality monitoring
//!
//! **NOTE** Eine Kopie aus embedded-hal project .Als Beispiel. Soll Bearbeitet und durch Documentation zu
//! wqa-analyzer ersetzt.
//!
//! - Must *erase* device specific details. Neither register, register blocks or magic values should
//! appear in the API.
//!
//! - Must be generic *within* a device and *across* devices. The API to use a serial interface must
//! be the same regardless of whether the implementation uses the USART1 or UART4 peripheral of a
//! device or the UART0 peripheral of another device.
//!
//! - Where possible must *not* be tied to a specific asynchronous model. The API should be usable
//! in blocking mode, with the `futures` model, with an async/await model or with a callback model.
//! (cf. the [`nb`] crate)
//!
//! - Must be minimal, and thus easy to implement and zero cost, yet highly composable. People that
//! want higher level abstraction should *prefer to use this HAL* rather than *re-implement*
//! register manipulation code.
//!
//! - Serve as a foundation for building an ecosystem of platform agnostic drivers. Here driver
//! means a library crate that lets a target platform interface an external device like a digital
//! sensor or a wireless transceiver. The advantage of this system is that by writing the driver as
//! a generic library on top of `embedded-hal` driver authors can support any number of target
//! platforms (e.g. Cortex-M microcontrollers, AVR microcontrollers, embedded Linux, etc.). The
//! advantage for application developers is that by adopting `embedded-hal` they can unlock all
//! these drivers for their platform.
//!
//! # Out of scope
//!
//! - Initialization and configuration stuff like "ensure this serial interface and that SPI
//! interface are not using the same pins". The HAL will focus on *doing I/O*.
//!
//! # Reference implementation
//!
//! The [`stm32f30x-hal`] crate contains a reference implementation of this HAL.
//!
//! [`stm32f30x-hal`]: https://crates.io/crates/stm32f30x-hal/0.1.0
//!
//! # Platform agnostic drivers
//!
//! You can find platform agnostic drivers built on top of `embedded-hal` on crates.io by [searching
//! for the *embedded-hal* keyword](https://crates.io/keywords/embedded-hal).
//!
//! If you writing a platform agnostic driver yourself you are highly encouraged to [add the
//! embedded-hal keyword](https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata)
//!
//! # Detailed design
//!
//! ## Traits
//!
//! The HAL is specified as traits to allow generic programming. These traits make use of the
//! [`nb`][] crate (*please go read that crate documentation before continuing*) to abstract over
//! the asynchronous model and to also provide a blocking operation mode.
//!
//! [`nb`]: https://crates.io/crates/nb
//!
//! Here's how a HAL trait may look like:
//!
//! ```
//! extern crate nb;
//!
//! /// A serial interface
//! pub trait Serial {
//!     /// Error type associated to this serial interface
//!     type Error;
//!
//!     /// Reads a single byte
//!     fn read(&mut self) -> nb::Result<u8, Self::Error>;
//!
//!     /// Writes a single byte
//!     fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error>;
//! }
//! ```
// #![deny(missing_docs)]
// #![deny(warnings)]
// #![no_std]


pub use embedded_hal as hal;

pub mod error;
pub mod gpio;
pub mod lamp;
pub mod pump;
pub mod valve;
pub mod sensor;
// pub mod ndir;
// pub mod uv;
pub mod mio;



// mod sensor;
// pub mod relay;
// pub mod valve;
// pub mod pump;
// pub mod autosampler;
// pub mod analog;
// pub mod lamp;
// pub mod sensor;
// pub mod uv;
// pub mod uhr;
// pub mod wecker;


// pub use temperatur::Temperatur;
// pub use sensor::NDir;
// pub use relay::Relay;
// pub use valve::Valve;
// pub use sensor::Sensor;
// pub use pump::Pump;
// pub use analog::Analog;
// pub use lamp::Lamp;
// pub use pressure::Pressure;
// pub use humidity::Humidity;
// pub use airflow::Airflow;

// pub use hid::*;
// pub use device::*;
// pub use indicators::*;
// pub use adjustment::*;
// pub use solution::*;
// pub use measurement::*;
// pub use integration::*;
// pub use statistic::*;


// pub use crate::uhr::Uhr;
// pub use crate::wecker::{Alarm, DayFlags, Wecker};
// pub use generic_array::ArrayLength;
// pub use gregor::{DateTime, FixedOffsetFromUtc, UnixTimestamp};

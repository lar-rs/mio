/// Relay

/// Single digital push-pull output pin
pub trait Relay {
    /// Error type
    type Error;

    /// Drives the on
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    fn open(&mut self) -> nb::Result<(), Self::Error>;

    /// Drives the off
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    fn close(&mut self) -> nb::Result<(), Self::Error>;
}

#[cfg(any(feature = "full-step", feature = "angular-velocity"))]
pub mod full;
#[cfg(feature = "debounced")]
pub mod half;

/// Direction Clockwise
pub const DIR_CW: u8 = 0x10;
/// Direction Counter Clockwise
pub const DIR_CCW: u8 = 0x20;

use crate::Time;

/// The duration of one second measured in record time.
pub const SECOND: Time = ffi::OMNI_TIME_SECOND;

/// The duration of one minute measured in record time.
pub const MINUTE: Time = ffi::OMNI_TIME_MINUTE;

/// The duration of one hour measured in record time.
pub const HOUR: Time = ffi::OMNI_TIME_HOUR;

// FIXME: The cast is necessary due to incorrect GIR type (RM #4199)
/// The duration of one millisecond measured in record time.
pub const MILLISECOND: Time = ffi::OMNI_TIME_MILLISECOND as _;

/// The duration of one microsecond measured in record time.
pub const MICROSECOND: Time = ffi::OMNI_TIME_MICROSECOND as _;

/// The duration of one nanosecond measured in record time.
pub const NANOSECOND: Time = ffi::OMNI_TIME_NANOSECOND as _;

// No-op tracing macros for use when tracing feature is not enabled.

macro_rules! debug {
    ($($arg:tt)*) => {};
}

macro_rules! warn_m {
    ($($arg:tt)*) => {};
}

pub(crate) use debug;
pub(crate) use warn_m as warn;

// No-op tracing macros for use when tracing feature is not enabled.

macro_rules! debug {
    ($($arg:tt)*) => {};
}

pub(crate) use debug;

pub trait Log: core::fmt::Write {}
pub trait DefaultLogger: Log + Default {}

type DefaultLoggerType = crate::arch::i686::logger::Logger;

pub fn default_logger() -> impl DefaultLogger {
    DefaultLoggerType::default()
}

#[macro_export]
macro_rules! log_raw_nonl {
    ($($args:tt)*) => {
        { let _ = write!(crate::log::default_logger(), $($args)*); }
    }
}

#[macro_export]
macro_rules! log_raw {
    ($($args:tt)*) => {
        {
            let mut logger = crate::log::default_logger();
            let _ = write!(logger, $($args)*);
            let _ = write!(logger, "\n");
        }
    }
}

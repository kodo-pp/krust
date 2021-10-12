pub trait Log: core::fmt::Write {}
pub trait DefaultLogger: Log + Default {}

type DefaultLoggerType = crate::arch::i686::logger::Logger;

pub fn default_logger() -> impl DefaultLogger {
    DefaultLoggerType::default()
}

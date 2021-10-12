use crate::log::{Log, DefaultLogger};
use core::fmt;
use super::serial::COM1;

#[derive(Debug, Copy, Clone, Default)]
pub struct Logger;

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        COM1.write_string(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        COM1.write_string("<write char>\n");
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        COM1.write_string("<write args>\n");
        Ok(())
    }
}

impl Log for Logger {}
impl DefaultLogger for Logger {}

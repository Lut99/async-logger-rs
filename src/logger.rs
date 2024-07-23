//  LOGGER.rs
//    by Lut99
//
//  Created:
//    23 Jul 2024, 05:37:32
//  Last edited:
//    23 Jul 2024, 06:34:55
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the logger itself.
//

use log::{Log, Metadata, Record};

use crate::runtimes::AsyncBackend;


/***** LIBRARY *****/
/// Defines a logger optimised for async usage in e.g. a webserver.
#[derive(Debug)]
pub struct AsyncLogger<A: AsyncBackend> {
    /// Defines the backend thread's worker.
    background: A::Worker,
}

// Log-impl
impl<A: AsyncBackend> Log for AsyncLogger<A> {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Check if it matches the level filter
        log::max_level() >= metadata.level()
    }

    fn log(&self, record: &Record) { todo!() }
    fn flush(&self) { todo!() }
}

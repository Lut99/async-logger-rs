//  ASYNC.rs
//    by Lut99
//
//  Created:
//    23 Jul 2024, 05:35:59
//  Last edited:
//    23 Jul 2024, 06:27:32
//  Auto updated?
//    Yes
//
//  Description:
//!   Abstraction over the asynchronous backend used by the crate.
//

// Declare modules
#[cfg(feature = "tokio")]
pub mod tokio;

// Imports
use log::Level;


/***** AUXILLARY *****/
/// An abstraction of log messages send to the background worker.
#[derive(Clone, Debug)]
pub struct Statement {
    /// The log level of this statement.
    pub level: Level,
}





/***** LIBRARY *****/
/// Defines a particular backend that can host [`AsyncWorker`]s.
pub trait AsyncBackend {
    /// Defines a (handle to a) background worker that processes log calls.
    type Worker: AsyncWorker;


    /// Spawns a new background worker for processing log events.
    ///
    /// # Returns
    /// A new [`Self::Worker`](AsyncBackend::Worker) that processes log events in the background.
    fn spawn(&mut self) -> Self::Worker;
}



/// Defines an abstraction over an async runtime.
pub trait AsyncWorker: Send + Sync {
    /// Sends a message for the worker to be logged
    ///
    /// # Arguments
    /// - `stmt`: The [`Statement`] to log.
    ///
    /// # Panics
    /// This function panics if it failed to emit the given `stmt`.
    async fn emit(&self, stmt: Statement);
}

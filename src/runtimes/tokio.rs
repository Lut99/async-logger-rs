//  TOKIO.rs
//    by Lut99
//
//  Created:
//    23 Jul 2024, 05:40:45
//  Last edited:
//    23 Jul 2024, 06:37:02
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the [`AsyncRuntime`] for [`tokio`].
//

use std::error;
use std::fmt::{Display, Formatter, Result as FResult};

use error_trace::ErrorTrace as _;
use log::LevelFilter;
use tokio::runtime::Handle;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

use super::{AsyncBackend, AsyncWorker, Statement};
use crate::writers::AsyncLogWriter;


/***** ERRORS *****/
/// Defines errors occurring from the [`TokioBackend`] or [`TokioWorker`].
#[derive(Debug)]
pub enum Error {
    /// Failed to grab the current tokio runtime.
    GetCurrent { err: tokio::runtime::TryCurrentError },
    /// Failed to emit a log message.
    EmitStatement { err: tokio::sync::mpsc::error::SendError<Statement> },
}
impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            GetCurrent { .. } => write!(f, "Failed to get current tokio runtime"),
            EmitStatement { .. } => write!(f, "Failed to emit log message to backend worker"),
        }
    }
}
impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            GetCurrent { err } => Some(err),
            EmitStatement { err } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Implements [`AsyncBackend`] for [`tokio`].
#[derive(Debug)]
pub struct TokioBackend<W> {
    /// The handle we schedule new things on.
    handle: Handle,
    /// The writer on which we do the writing.
    writer: W,
}

// Constructors
impl<W: Default> Default for TokioBackend<W> {
    #[inline]
    fn default() -> Self { Self::current(W::default()) }
}
impl<W> TokioBackend<W> {
    /// Creates a new TokioBackend that uses the tokio runtime currently in context.
    ///
    /// # Arguments
    /// - `writer`: The writer to which to write the log messages.
    ///
    /// # Returns
    /// A [`TokioBackend::Context`] that will use the local context for spawning background processes.
    #[inline]
    pub fn current(writer: W) -> Self {
        match Handle::try_current() {
            Ok(handle) => Self { handle, writer },
            Err(err) => panic!("{}", Error::GetCurrent { err }.trace()),
        }
    }

    /// Creates a new TokioBackend around the given runtime handle.
    ///
    /// # Arguments
    /// - `handle`: The tokio [`Handle`] referring to the runtime we're using.
    /// - `writer`: The writer to which to write the log messages.
    ///
    /// # Returns
    /// A [`TokioBackend::Handle`] that will use the given handle for spawning background processes.
    #[inline]
    pub const fn handle(handle: Handle, writer: W) -> Self { Self { handle, writer } }
}

// AsyncBackend impl
impl<W: 'static + Send + Sync + Clone + AsyncLogWriter> AsyncBackend for TokioBackend<W> {
    type Worker = TokioWorker;


    #[inline]
    fn spawn(&mut self) -> Self::Worker {
        // Create the channels
        let (sender, receiver): (Sender<Statement>, Receiver<Statement>) = tokio::sync::mpsc::channel(16);

        // Schedule the task there
        TokioWorker { _handle: self.handle.spawn(TokioWorker::work::<W>(receiver, self.writer.clone())), sender }
    }
}



/// Implements a handle to a [`TokioBackend`] worker implementing [`AsyncWorker`].
#[derive(Debug)]
pub struct TokioWorker {
    /// The handle to the actual task.
    _handle: JoinHandle<()>,
    /// A sender with a queue for log messages.
    sender:  Sender<Statement>,
}
impl TokioWorker {
    /// The body of the worker.
    ///
    /// # Arguments
    /// - `receiver`: The [`Receiver`] to receive new [`Statements`] to log on.
    async fn work<W: AsyncLogWriter>(mut receiver: Receiver<Statement>, mut writer: W) {
        loop {
            // Get the next message
            let msg: Statement = match receiver.recv().await {
                Some(msg) => msg,
                None => break,
            };

            // See if we should log the message
            let current: LevelFilter = log::max_level();
            if current < msg.level {
                // Nothing to do
                continue;
            }

            // Write the initial message
        }
    }
}
impl AsyncWorker for TokioWorker {
    #[inline]
    async fn emit(&self, stmt: Statement) {
        if let Err(err) = self.sender.send(stmt).await {
            panic!("{}", Error::EmitStatement { err }.trace());
        }
    }
}

//  TOKIO.rs
//    by Lut99
//
//  Created:
//    23 Jul 2024, 06:37:39
//  Last edited:
//    23 Jul 2024, 06:43:20
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a file writer that plays nice with tokio.
//

use std::sync::Arc;

use tokio::fs::File;
use tokio::sync::Mutex;

use super::AsyncLogWriter;


/***** LIBRARY *****/
/// Implements a writer for log messages to the terminal.
#[derive(Clone, Debug)]
pub struct TokioFileWriter {
    /// The handle we're writing to.
    handle: Arc<Mutex<File>>,
    /// Whether color should be used.
    color:  bool,
}

// Constructors
impl TokioFileWriter {
    /// Creates a new TokioFileWriter.
    /// 
    /// # Arguments
    /// - `path`: The path to create the file writer on.
    /// 
    /// # Returns
    /// A new 
}

// AsyncLogWriter impl
impl AsyncLogWriter for TokioFileWriter {
    #[inline]
    fn use_color() -> bool { false }
}

//  MOD.rs
//    by Lut99
//
//  Created:
//    23 Jul 2024, 06:34:34
//  Last edited:
//    23 Jul 2024, 06:39:58
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines possible writers for log messages.
//

#[cfg(feature = "tokio")]
pub mod tokio;


/***** LIBRARY *****/
/// Defines an asynchronous log writer.
pub trait AsyncLogWriter {
    /// Whether this output (should) support color.
    ///
    /// # Returns
    /// True if colors are desired, or false otherwise.
    fn use_color() -> bool;
}

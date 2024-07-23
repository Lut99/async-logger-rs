//  LIB.rs
//    by Lut99
//
//  Created:
//    23 Jul 2024, 05:35:37
//  Last edited:
//    23 Jul 2024, 06:34:24
//  Auto updated?
//    Yes
//
//  Description:
//!   A logger for Rust projects relying on tokio parallelism - e.g.,
//!   webservers.
//

// Declare the modules
mod logger;
mod runtimes;
mod writers;

// Expose the public API
pub use logger::AsyncLogger;

//! This module contains English phrases.
//! 
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}",
//!     odds_and_ends::greetings::english::hello(), username);
//! 
//! ```
//! -> (not ignored by the compiler)

// once documentation is written run rustdoc <filename>.rs 

// comment (ignored by the compiler)

/*
    BIG COMMENTS (ignored by the compiler)
*/

/// Applies to code that follows it
/// In this cas it's our `hello()` function.
/// _Markdown is handled by the documentation comments._
/// -> (not ignored by the compiler)
pub fn hello() -> String { "hello!".to_string() }


pub fn bye() -> String { "bye!".to_string() }
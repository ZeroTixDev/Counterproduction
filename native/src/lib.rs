#![feature(arbitrary_self_types)]
//! The native component of the game.
//!
//! This will consist of everything apart from the user input and UI.

/// The main code that runs the rust end.
pub mod main;
/// All block related stuff, including storage of blocks.
pub mod blocks;
/// All entity related stuff.
pub mod entity;
/// All rendering related stuff. This currently only includes a small mesh implementation that gets sent to the JavaScript part.
pub mod rendering;
/// Random stuff that doesn't belong anywhere else.
pub mod utils;

#[cfg(test)]
mod tests;

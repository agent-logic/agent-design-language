//! CodeFriend product entrypoints. Repository content is inert evidence.
pub mod ingestion;
pub mod integration;

pub mod actions;
pub mod activities;
pub mod architecture;
pub mod evidence;
pub mod governance;
pub mod memory;
pub mod operator;
pub mod publication;
pub mod review;

pub mod server;

pub mod agent;

pub mod language;
mod rust_parse;
pub(crate) mod schema;

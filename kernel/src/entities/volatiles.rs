//! This module is a collection of structures
//! that define volatiles and temporary data,
//! intended to be handled in an in-memory database such as Redis.

mod ticket;
mod mfa_code;
mod pkce;
mod state;

pub use self::{
    ticket::*,
    mfa_code::*,
    pkce::*,
    state::*
};
//! Event Tracing for Windows: starts sessions, enables providers, consumes
//! their events and reads their fields.
//!
//! Nothing of windows-rs crosses this crate's API: a provider is a `u128`,
//! an event an [`Event`], a failure an [`Error`]. Its callers need not take
//! windows-rs from the same place as it does.

mod aligned;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]
mod bindings;
mod error;
mod event;
mod privilege;
mod schema;
mod session;
mod trace;

pub use error::{Error, Result};
pub use event::{Event, OwnedEvent};
pub use session::{Counters, Enable, LogFile, Options, Session, running, stop};
pub use trace::{Clock, ClockKind, Timestamps, Trace, read_file};

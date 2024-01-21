//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{Error, Result};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

// region:    --- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String
}
// endregion: --- Ticket Types


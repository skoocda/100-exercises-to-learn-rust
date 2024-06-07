use crate::data::{Status, Ticket, TicketDraft};

use std::collections::BTreeMap;

//These will change
use std::sync::{Arc, RwLock};

#[derive( Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>, // TODO: needs to change
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        todo!();
    }

    pub fn update_ticket(&mut self, ticket: Ticket) -> TicketId {
        todo!();
    }

    pub fn get(&self, id:TicketId) -> Option<Ticket> { // TODO: needs new return type
        self.ticket.get(&id).cloned()
    }
}
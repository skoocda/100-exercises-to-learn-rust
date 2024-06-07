// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: unimplemented!() // ?
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, mut receiver) = mpsc::channel(32);

    //sender can be cloned

    tokio::spawn(async move {
        server(receiver)
    })
}

enum Command {
    Insert {
        draft: TicketDraft,
        //response_channel: unimplemented!(),
    },
    Get {
        id: TicketId,
        //response_channel: unimplemented!(),
    },
    // TODO: Update
}

pub fn server(receiver: Receiver) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv().await {
            Some(message) => todo!(),
            Err(e) => eprintln!("Issue with server!", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};


    async fn test_insert() {
        //server launch
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(launch(listener));

        //client launch
        let request = Command::Insert {
            draft: TicketDraft {
                title: ticket_title(),
                description: ticket_description(),
            },
        };
        let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
        let (mut reader, mut writer) = socket.split();

        // need to serialize TicketDraft into a bytestream
        // could use the tokio utils encoder?

        //write request
        writer.write_all(request.as_bytes()).await.unwrap();
        writer.shutdown().await.unwrap();

        //receive response
        let mut buf = Vec::with_capacity(request.len());
        reader.read_to_end(&mut buf).await.unwrap();
        
        // need to deserialize into a TicketId

        assert_eq!(true, true);

    }

    fn works() {
        let client = launch(32);
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let ticket_id = todo!();

        let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();

    }
}
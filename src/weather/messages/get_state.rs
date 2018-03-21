extern crate actix;

use weather;

pub struct Message;

impl actix::Message for Message {
    type Result = Result<weather::structs::State, ()>;
}

impl actix::Handler<Message> for weather::Actor {
    type Result = Result<weather::structs::State, ()>;

    fn handle(&mut self, _: Message, _: &mut actix::Context<Self>) -> Self::Result {
        Ok(self.state.clone())
    }
}
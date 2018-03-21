extern crate actix;

use super::Actor;

impl actix::Actor for Actor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.process(ctx);
    }
}
use futures::{done, Future};
use Statement;
use Response;
use Error;

pub struct Session {
}

impl Session {
    pub fn set_keyspace(&mut self, keyspace: &str) ->  Response {
        done(Err(Error::Undefined)).boxed()
    }
    pub fn statement(stmt: &str) -> Statement {
        Statement{}
    }

    pub fn execute(&self,  stmt: &Statement) -> Response {
        done(Err(Error::Undefined)).boxed()
    }

}

use futures::{done, Future};
use Statement;
use Response;
use CqlError;

use Rows;

pub struct Session {
}

impl Session {
    pub fn set_keyspace(&mut self, keyspace: &str) ->  Response<()> {
        done(Err(CqlError::Undefined)).boxed()
    }
    pub fn statement(stmt: &str) -> Statement {
        Statement{}
    }

    pub fn execute(&self,  stmt: &Statement) -> Response<Rows> {
        done(Err(CqlError::Undefined)).boxed()
    }

}

use futures::BoxFuture;
use Value;
use Error;

pub type Response = BoxFuture<Value, Error>;

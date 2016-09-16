use futures::BoxFuture;
use Value;
use CqlError;

pub type Response<T> = BoxFuture<T, CqlError>;

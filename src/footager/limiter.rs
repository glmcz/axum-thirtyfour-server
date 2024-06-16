use std::task::{Context, Poll};
use axum::extract::Request;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct MyLayer;

impl MyLayer {
    pub fn new() -> Self {
        Self {}
    }
}impl<S> Layer<S> for MyLayer {
    type Service = Limiter<S>;
    fn layer(&self, inner: S) -> Self::Service {
        println!("Middleware hello world");
        Limiter { inner }
    }
}
#[derive(Clone)]
pub struct Limiter<S> {
    inner: S,
}
impl<S, B> Service<Request<B>> for Limiter<S>
    where
        S: Service<Request<B>> + Clone + Send
{
    type Response = S::Response;

    type Error = S::Error;
    type Future = S::Future;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Our timeout service is ready if the inner service is ready.
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        println!("middleware hello world");
        self.inner.call(req)
    }
}

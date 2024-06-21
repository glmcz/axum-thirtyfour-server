use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, ready};
use std::time::Duration;
use axum::extract::Request;
use tokio_util::sync::PollSemaphore;
use tokio::sync::{OwnedSemaphorePermit, Semaphore, SemaphorePermit};

use tower::{Layer, Service, ServiceExt};
use pin_project::pin_project;

#[pin_project]
pub struct ResponseFuture<T> {
    #[pin]
    response_future: T,
    #[pin]
    permit: OwnedSemaphorePermit,
}
impl<T> ResponseFuture<T>{
    fn new(response_future: T, permit: OwnedSemaphorePermit) -> ResponseFuture<T> {
        ResponseFuture{ response_future, permit }
    }
}

impl<F, T, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<T, E>>
{
    type Output = Result<T, E>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // It checks for completion of the actual service call.
        // ****************************************************
        let this = self.project();
        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }

        // match this.permit.(cx){
        //     Poll::Ready(result) => {
        //         let result = result.map_err(Into::into);
        //         return Poll::Ready(result);
        //     }
        //     Poll::Pending => {}
        // }

        Poll::Pending
    }
}


#[derive(Clone)]
pub struct MyLayer{
    limit: usize,
}
impl MyLayer {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }
}
impl<S> Layer<S> for MyLayer {
    type Service = Limiter<S>;
    fn layer(&self, inner: S) -> Self::Service {
        Limiter::new(inner, self.limit)
    }
}

pub struct Limiter<S> {
    inner: S,
    semaphore: PollSemaphore,
    permit: Option<OwnedSemaphorePermit>,
}
impl<S> Limiter<S> {
    pub fn new(inner: S, limit: usize) -> Self {
        Limiter{
            inner,
            semaphore: PollSemaphore::new(Arc::new(Semaphore::new(limit))),
            permit: None,
        }
    }
}

impl<S: Clone> Clone for Limiter<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            semaphore: self.semaphore.clone(),
            permit: None,
        }
    }
}
impl<S, Request> Service<Request> for Limiter<S>
    where
        S: Service<Request>
{
    type Response = S::Response;

    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // poll make sure, that only defined num of permits go into
        // our service handler fn. If handler is busy permits are freed
        // only after handler finish his jobs.

        // we can also use
        let num = self.semaphore.available_permits();
        println!("available {}", num);
        match self.semaphore.poll_acquire(cx) {
            Poll::Ready(Some(permit)) => {
                println!("Acquired a permit");
                self.permit = Some(permit);
                Poll::Ready(Ok(()))
            }
            Poll::Ready(None) => {
                Poll::Pending
            }
            Poll::Pending => {
                // it goes there only once for each req
                Poll::Pending // waking up is a job of executor (Tokio)
                // actualy if poll is pending we can do something different.
                // for example check CPU, load balancing, etc...
            }
        }
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let future = self.inner.call(req);
        println!("middleware hello world");
        let permit = self
            .permit
            .take()
            .expect("max req is reached. poll_ready must be called first");
        println!("middleware hello world");
        ResponseFuture::new(future, permit)
    }
}

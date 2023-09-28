use crate::*;
use std::future::{ready, Ready};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, forward_ready, Transform};
use actix_web::Error;
use futures_util::future::LocalBoxFuture;

struct MidWare;

impl<S, B> Transform<S, ServiceRequest> for MidWare
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TestMWMidWare<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TestMWMidWare { service }))
    }
}

struct TestMWMidWare<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TestMWMidWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("hi from start, you requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("hi from response");
            Ok(res)
        })
    }
}
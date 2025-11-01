use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures::future::{LocalBoxFuture, Ready, ok};
use std::task::{Context, Poll};
use tracing::info_span;
use uuid::Uuid;

pub const CORRELATION_ID: &str = "X-Correlation-ID";

pub struct CorrelationIdMiddleware;

impl<S, B> Transform<S, ServiceRequest> for CorrelationIdMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CorrelationIdMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CorrelationIdMiddlewareInner { service })
    }
}

pub struct CorrelationIdMiddlewareInner<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CorrelationIdMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let correlation_id = req
            .headers()
            .get(CORRELATION_ID)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        req.extensions_mut().insert(correlation_id.clone());

        let span = info_span!("http_request", %correlation_id);

        let fut = self.service.call(req);
        Box::pin(async move {
            let _guard = span.enter();
            let res = fut.await?;
            Ok(res)
        })
    }
}

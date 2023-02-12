use std::future::{ready, Ready};
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, error::{ErrorUnauthorized}, Error};
use actix_web::dev::forward_ready;
use actix_web::http::header::{HeaderName, HeaderValue};
use futures::future::{LocalBoxFuture};
use futures::FutureExt;

const API_KEY_HEADER_NAME: HeaderName = HeaderName::from_static("x-api-key");

#[derive(Clone)]
pub struct ApiKeyGuard {
  pub api_key: String,
}


impl<S, B> Transform<S, ServiceRequest> for ApiKeyGuard
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = ApiKeyMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(ApiKeyMiddleware {
      service,
      api_key_value: HeaderValue::from_str(&self.api_key).unwrap()
    }))
  }
}

pub struct ApiKeyMiddleware<S> {
  service: S,
  api_key_value: HeaderValue,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    if let Some(header_value) = req.headers().get(API_KEY_HEADER_NAME) {
      if header_value.eq(&self.api_key_value) {
        return self.service.call(req).boxed_local();
      }
    }

    return ready(Err(ErrorUnauthorized("unauthorized"))).boxed_local();
  }
}

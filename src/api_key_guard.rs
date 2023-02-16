use crate::config::get_prefixed_env;
use actix_web::dev::forward_ready;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{
  dev::{Service, ServiceRequest, ServiceResponse, Transform},
  error::ErrorUnauthorized,
  Error,
};
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use std::future::{ready, Ready};

const API_KEY_HEADER_NAME: HeaderName = HeaderName::from_static("x-api-key");

#[derive(Clone)]
pub struct ApiKeyGuard {
  pub api_key: Option<String>,
}

impl Default for ApiKeyGuard {
  fn default() -> Self {
    ApiKeyGuard {
      api_key: match get_prefixed_env("API_KEY") {
        Some(value) => Some(value),
        None => {
          log::warn!("no api key provided");
          None
        }
      },
    }
  }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyGuard
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
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
      api_key_value: self.api_key.clone().map(|value| HeaderValue::from_str(&value).unwrap()),
    }))
  }
}

pub struct ApiKeyMiddleware<S> {
  service: S,
  api_key_value: Option<HeaderValue>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
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
    return match (&self.api_key_value, req.headers().get(API_KEY_HEADER_NAME)) {
      (Some(api_key_value), Some(header_value)) => {
        if api_key_value == header_value {
          self.service.call(req).boxed_local()
        } else {
          ready(Err(ErrorUnauthorized("unauthorized"))).boxed_local()
        }
      }
      (Some(_), None) => ready(Err(ErrorUnauthorized("unauthorized"))).boxed_local(),
      (None, _) => self.service.call(req).boxed_local(),
    };
  }
}

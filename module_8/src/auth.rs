use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::dev::{Service, Transform};
use futures_util::future::{ok, Ready};
use futures_util::FutureExt;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use std::rc::Rc;
use std::task::{Context, Poll};

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                let decoding_key = DecodingKey::from_secret("secret".as_ref());
                let validation = Validation::new(Algorithm::HS256);

                if let Ok(token_data) = decode::<Claims>(token, &decoding_key, &validation) {
                    req.extensions_mut().insert(token_data.claims);
                    return self.service.call(req).boxed_local();
                }
            }
        }

        let response = req.into_response(
            actix_web::HttpResponse::Unauthorized().finish().into_body()
        );
        futures_util::future::ok(response).boxed_local()
    }
}

use super::search;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};

use std::future::Future;
use std::pin::Pin;

// What are these derive decorators?
// Seems like they apply properties to the below struct defn
#[derive(Debug, Clone)]
pub struct Svc {}

// Copied from the hyper docs
// https://github.com/hyperium/hyper/blob/master/examples/service_struct_impl.rs
impl Service<Request<IncomingBody>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            let url = "http://httpbin.org/ip".parse::<hyper::Uri>()?;
            search::call(url);
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        let res = match req.uri().path() {
            "/hello" => mk_response(format!("hit it!")),
            path => mk_response(format!("{} not found", path)),
        };

        Box::pin(async { res })
    }
}

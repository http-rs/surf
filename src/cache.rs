use std::collections::HashMap;

use cacache;
use futures::prelude::*;
use http::Request;
use hyper;
use serde_json::{Map, Number, Value};

use crate::response::Response;
use crate::Fail;

pub fn is_cacheable<T>(req: &Request<T>) -> bool {
    unimplemented!();
}

fn cache_key<T>(req: &Request<T>) -> String {
    format!("surf:req:v1:{}", req.uri())
}

#[derive(Debug)]
pub struct Cacache {
    path: String,
}

impl Cacache {
    pub async fn matched<T, U>(&self, req: &Request<T>) -> Result<Option<Response<U>>, Fail>
    where
        U: AsyncRead,
    {
        if let Some(entry) = cacache::get::info(&self.path, cache_key(&req))? {
            let mut res = hyper::Response::builder();
            // TODO - convert the unwrap()s to .ok_or()
            res.status(entry.metadata["status"].as_u64().unwrap() as u16);
            let headers = entry.metadata["headers"].as_object().unwrap();
            for (header, value) in headers.into_iter() {
                res.header(header.as_str(), value.as_str().unwrap());
            }
            Ok(Some(Response::new(
                res.body(hyper::Body::empty())?,
                cacache::async_get::open_hash(&self.path, entry.integrity).await?,
            )))
        } else {
            Ok(None)
        }
    }

    pub async fn put<T, U>(&self, req: Request<T>, res: Response<U>) -> Response<U>
    where
        U: AsyncRead,
    {
        let mut metadata = Map::new();
        metadata.insert(
            "status".into(),
            Value::Number(Number::from(res.response.status().as_u16())),
        );
        let metadata = Value::Object(metadata);
        let put_opts = cacache::put::PutOpts::new().metadata(metadata);
    }

    pub async fn delete<T>(&self, req: Request<T>) -> Result<bool, Fail> {
        unimplemented!();
    }
}

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

    pub async fn put<T, U>(&self, req: Request<T>, res: Response<U>) -> Result<Response<U>, Fail>
    where
        U: AsyncRead + Unpin,
    {
        let mut metadata = Map::new();
        metadata.insert(
            "status".into(),
            Value::Number(Number::from(res.response.status().as_u16())),
        );
        let mut headers = Map::new();
        for (key, value) in res.response.headers().iter() {
            headers.insert(key.as_str().into(), Value::String(value.to_str()?.into()));
        }
        metadata.insert("headers".into(), Value::Object(headers));
        let metadata = Value::Object(metadata);
        let put = cacache::put::PutOpts::new()
            .metadata(metadata)
            .open_async(&self.path, cache_key(&req))
            .await?;
        let mut buf = [0; 1024 * 256];
        loop {
            let amt = res.read(&mut buf).await?;
            if amt == 0 {
                break;
            } else {
                put.write_all(&buf[0..amt]).await?;
            }
        }
        put.commit().await?;
        let res = self.matched(&req).await?.unwrap();
        Ok(res)
    }

    pub async fn delete<T>(&self, req: Request<T>) -> Result<bool, Fail> {
        unimplemented!();
    }
}

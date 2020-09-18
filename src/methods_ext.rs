use crate::RequestBuilder;

/// Extension trait that adds http request methods
///
/// Blanket implementation provided for all `http_client::HttpClient`s
pub trait MethodsExt {
    /// Construct a new surf Client
    fn client(&self) -> surf::Client;

    /// Builds a `CONNECT` request.
    fn connect(&self, path: &str) -> RequestBuilder {
        self.client().connect(path)
    }

    /// Builds a `DELETE` request.
    fn delete(&self, path: &str) -> RequestBuilder {
        self.client().delete(path)
    }

    /// Builds a `GET` request.
    fn get(&self, path: &str) -> RequestBuilder {
        self.client().get(path)
    }

    /// Builds a `HEAD` request.
    fn head(&self, path: &str) -> RequestBuilder {
        self.client().head(path)
    }

    /// Builds an `OPTIONS` request.
    fn options(&self, path: &str) -> RequestBuilder {
        self.client().options(path)
    }

    /// Builds a `PATCH` request.
    fn patch(&self, path: &str) -> RequestBuilder {
        self.client().patch(path)
    }

    /// Builds a `POST` request.
    fn post(&self, path: &str) -> RequestBuilder {
        self.client().post(path)
    }

    /// Builds a `PUT` request.
    fn put(&self, path: &str) -> RequestBuilder {
        self.client().put(path)
    }

    /// Builds a `TRACE` request.
    fn trace(&self, path: &str) -> RequestBuilder {
        self.client().trace(path)
    }
}

impl<HC: http_client::HttpClient + Clone> MethodsExt for HC {
    fn client(&self) -> Client {
        Client::with_http_client(std::sync::Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use futures_util::future::BoxFuture;
    use http_types::{Body, Error, Request, Response};

    #[async_std::test]
    async fn with_a_fake_client() -> http_types::Result<()> {
        #[derive(Debug, Clone)]
        struct MyClient;
        impl http_client::HttpClient for MyClient {
            fn send(&self, _req: Request) -> BoxFuture<'static, Result<Response, Error>> {
                Box::pin(async move {
                    let mut response = Response::new(200);
                    response.set_body(Body::from_string(String::from("hello")));
                    Ok(response)
                })
            }
        }
        use super::MethodsExt;
        let mut response = MyClient.get("http://hello.example").await?;
        assert_eq!(response.body_string().await?, "hello");
        assert!(response.status().is_success());

        Ok(())
    }
}

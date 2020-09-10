use crate::http::Method;
use crate::RequestBuilder;

/// Perform a one-off `GET` request.
///
/// # About the HTTP Method
///
/// The GET method requests a representation of the specified resource. Requests using GET should
/// only retrieve data.
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::get("https://httpbin.org/get").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn get(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Get, uri)
}

/// Perform a one-off `HEAD` request.
///
/// # About the HTTP Method
///
/// The HTTP HEAD method requests the headers that are returned if the specified resource would be
/// requested with an HTTP GET method. Such a request can be done before deciding to download a
/// large resource to save bandwidth, for example.
///
/// A response to a HEAD method should not have a body. If so, it must be ignored. Even so, entity
/// headers describing the content of the body, like Content-Length may be included in the
/// response. They don't relate to the body of the HEAD response, which should be empty, but to the
/// body of similar request using the GET method would have returned as a response.
///
/// If the result of a HEAD request shows that a cached resource after a GET request is now
/// outdated, the cache is invalidated, even if no GET request has been made.
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/HEAD
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::head("https://httpbin.org/head").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn head(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Head, uri)
}

/// Perform a one-off `POST` request.
///
/// # About the HTTP Method
///
/// The HTTP POST method sends data to the server. The type of the body of the request is indicated
/// by the Content-Type header.
///
/// The difference between PUT and POST is that PUT is idempotent: calling it once or several times
/// successively has the same effect (that is no side effect), where successive identical POST may
/// have additional effects, like passing an order several times.
///
/// A POST request is typically sent via an HTML form and results in a change on the server. In
/// this case, the content type is selected by putting the adequate string in the enctype attribute
/// of the `<form>` element or the formenctype attribute of the `<input>` or `<button>` elements:
///
/// ```txt
/// application/x-www-form-urlencoded: the keys and values are encoded in key-value tuples separated by '&', with a '=' between the key and the value. Non-alphanumeric characters in both keys and values are percent encoded: this is the reason why this type is not suitable to use with binary data (use multipart/form-data instead)
/// multipart/form-data: each value is sent as a block of data ("body part"), with a user agent-defined delimiter ("boundary") separating each part. The keys are given in the Content-Disposition header of each part.
/// text/plain
/// ```
///
/// When the POST request is sent via a method other than an HTML form — like via an XMLHttpRequest
/// — the body can take any type. As described in the HTTP 1.1 specification, POST is designed to
/// allow a uniform method to cover the following functions:
///
/// ```txt
/// Annotation of existing resources
/// Posting a message to a bulletin board, newsgroup, mailing list, or similar group of articles;
/// Adding a new user through a signup modal;
/// Providing a block of data, such as the result of submitting a form, to a data-handling process;
/// Extending a database through an append operation.
/// ```
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::post("https://httpbin.org/post").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn post(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Post, uri)
}

/// Perform a one-off `PUT` request.
///
/// # About the HTTP Method
///
/// The HTTP PUT request method creates a new resource or replaces a representation of the target
/// resource with the request payload.
///
/// The difference between PUT and POST is that PUT is idempotent: calling it once or several times
/// successively has the same effect (that is no side effect), where successive identical POST may
/// have additional effects, like passing an order several times.
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/PUT
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::put("https://httpbin.org/put").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn put(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Put, uri)
}

/// Perform a one-off `DELETE` request.
///
/// # About the HTTP Method
///
/// The HTTP DELETE request method deletes the specified resource.
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/DELETE
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::delete("https://httpbin.org/delete").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn delete(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Delete, uri)
}

/// Perform a one-off `CONNECT` request.
///
/// # About the HTTP Method
///
/// The HTTP CONNECT method method starts two-way communications with the requested resource. It
/// can be used to open a tunnel.
///
/// For example, the CONNECT method can be used to access websites that use SSL (HTTPS). The client
/// asks an HTTP Proxy server to tunnel the TCP connection to the desired destination. The server
/// then proceeds to make the connection on behalf of the client. Once the connection has been
/// established by the server, the Proxy server continues to proxy the TCP stream to and from the
/// client.
///
/// CONNECT is a hop-by-hop method.
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/CONNECT
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::connect("https://httpbin.org/connect").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn connect(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Connect, uri)
}

/// Perform a one-off `OPTIONS` request.
///
/// # About the HTTP Method
///
/// The HTTP OPTIONS method is used to describe the communication options for the target resource.
/// The client can specify a URL for the OPTIONS method, or an asterisk (*) to refer to the entire
/// server.
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/OPTIONS
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::options("https://httpbin.org/options").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn options(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Options, uri)
}

/// Perform a one-off `TRACE` request.
///
/// # About the HTTP Method
///
/// The HTTP TRACE method performs a message loop-back test along the path to the target resource,
/// providing a useful debugging mechanism.
///
/// The final recipient of the request should reflect the message received, excluding some fields
/// described below, back to the client as the message body of a 200 (OK) response with a
/// Content-Type of message/http. The final recipient is either the origin server or the first
/// server to receive a Max-Forwards value of 0 in the request.
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/TRACE
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::trace("https://httpbin.org/trace").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn trace(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Trace, uri)
}

/// Perform a one-off `PATCH` request.
///
/// # About the HTTP Method
///
/// The HTTP PATCH request method applies partial modifications to a resource.
///
/// The HTTP PUT method only allows complete replacement of a document. Unlike PUT, PATCH is not
/// idempotent, meaning successive identical patch requests may have different effects. However, it
/// is possible to issue PATCH requests in such a way as to be idempotent.
///
/// PATCH (like PUT) may have side-effects on other resources.
///
/// To find out whether a server supports PATCH, a server can advertise its support by adding it to
/// the list in the Allow or Access-Control-Allow-Methods (for CORS) response headers.
///
/// Another (implicit) indication that PATCH is allowed, is the presence of the Accept-Patch
/// header, which specifies the patch document formats accepted by the server.
///
/// [Read more on MDN]
///
/// [Read more on MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/PATCH
///
/// # Panics
///
/// This will panic if a malformed URL is passed.
///
/// # Errors
///
/// Returns errors from the middleware, http backend, and network sockets.
///
/// # Examples
///
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let string = surf::patch("https://httpbin.org/patch").recv_string().await?;
/// # Ok(()) }
/// ```
pub fn patch(uri: impl AsRef<str>) -> RequestBuilder {
    let uri = uri.as_ref().parse().unwrap();
    RequestBuilder::new(Method::Patch, uri)
}

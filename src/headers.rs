//! HTTP Headers.

use std::iter::{IntoIterator, Iterator};

/// A collection of HTTP Headers.
#[derive(Debug)]
pub struct Headers<'a> {
    headers: &'a mut http::HeaderMap,
}

impl<'a> Headers<'a> {
    /// Create a new instance.
    pub(crate) fn new(headers: &'a mut http::HeaderMap) -> Self {
        Self { headers }
    }

    /// Get a header.
    pub fn get<K>(&self, key: K) -> Option<&'_ str>
    where
        K: http::header::AsHeaderName,
    {
        self.headers.get(key).map(|h| h.to_str().unwrap())
    }

    /// Set a header.
    pub fn insert<K>(&mut self, key: K, value: impl AsRef<str>) -> Option<String>
    where
        K: http::header::IntoHeaderName,
    {
        let value = value.as_ref().to_owned();
        let res = self.headers.insert(key, value.parse().unwrap());
        res.as_ref().map(|h| h.to_str().unwrap().to_owned())
    }

    /// Iterate over all headers.
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.headers.iter())
    }
}

impl<'a> IntoIterator for Headers<'a> {
    type Item = (&'a str, &'a str);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.headers.iter())
    }
}

/// An iterator over headers in `Headers`.
#[derive(Debug)]
pub struct Iter<'a>(http::header::Iter<'a, http::header::HeaderValue>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(key, value)| (key.as_str(), value.to_str().unwrap()))
    }
}

pub struct ReadOnlyHeaderMap {
    inner: actix_http::header::HeaderMap,
}

impl ReadOnlyHeaderMap {

    pub(crate) fn new(inner: actix_http::header::HeaderMap) -> Self {
        Self { inner }
    }

    pub fn keys(&self) -> Vec<&str> {
        self.inner.keys().map(|k| k.as_str()).collect()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
        if let Some(header_value) = self.inner.get(key.as_ref()) {
            Some(header_value.to_str().unwrap())
        } else {
            None
        }
    }
}
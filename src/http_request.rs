use std::sync::Arc;

use crate::ffi::{HttpRequestFfi, RealHttpRequestFfi};

pub(crate) struct HttpRequestCore<THttpRequestFfi>
where
    THttpRequestFfi: HttpRequestFfi,
{
    http_request_ffi: Arc<THttpRequestFfi>,
}

impl<THttpRequestFfi> HttpRequestCore<THttpRequestFfi>
where
    THttpRequestFfi: HttpRequestFfi,
{
    pub(crate) fn new(http_request_ffi: Arc<THttpRequestFfi>) -> Self {
        HttpRequestCore { http_request_ffi }
    }

    pub(crate) fn http_request_ffi(&self) -> Arc<THttpRequestFfi> {
        Arc::clone(&self.http_request_ffi)
    }

    pub(crate) fn process(&self) -> i32 {
        self.http_request_ffi.process_request()
    }
}

// This is the public API exposed library consumers.
// It is a wrapper that uses real FFI for everything underneath.
pub struct HttpRequest(HttpRequestCore<RealHttpRequestFfi>);

// TODO: We could perhaps simplify this newtype generation via an attribute macro on the impl block.
// This macro could automatically generate the necessary wrapper functions.
impl HttpRequest {
    pub fn new() -> Self {
        HttpRequest(HttpRequestCore::new(
            Arc::new(RealHttpRequestFfi::default()),
        ))
    }

    pub fn process(&self) -> i32 {
        self.0.process()
    }
}

// Boilerplate transform between core/public types.
impl From<HttpRequest> for HttpRequestCore<RealHttpRequestFfi> {
    fn from(http_request: HttpRequest) -> Self {
        http_request.0
    }
}

// Boilerplate transform between core/public types.
impl From<HttpRequestCore<RealHttpRequestFfi>> for HttpRequest {
    fn from(http_request_core: HttpRequestCore<RealHttpRequestFfi>) -> Self {
        HttpRequest(http_request_core)
    }
}

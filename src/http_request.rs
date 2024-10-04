use crate::ffi::{HttpRequestFfi, RealHttpRequestFfi};

pub(crate) struct HttpRequestCore<THttpRequestFfi>
where
    THttpRequestFfi: HttpRequestFfi + 'static,
{
    pub(crate) http_request_ffi: &'static THttpRequestFfi,
}

impl<THttpRequestFfi> HttpRequestCore<THttpRequestFfi>
where
    THttpRequestFfi: HttpRequestFfi + 'static,
{
    pub(crate) fn new(http_request_ffi: &'static THttpRequestFfi) -> Self {
        HttpRequestCore { http_request_ffi }
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
            &RealHttpRequestFfi,
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

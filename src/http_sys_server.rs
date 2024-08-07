use std::sync::Arc;

use crate::{
    ffi::{HttpRequestFfi, HttpSysFfi, RealHttpRequestFfi, RealHttpSysFfi},
    http_request::{HttpRequest, HttpRequestCore},
};

pub(crate) struct HttpSysServerCore<THttpSysFfi, THttpRequestFfi>
where
    THttpSysFfi: HttpSysFfi,
    THttpRequestFfi: HttpRequestFfi,
{
    http_sys_ffi: Arc<THttpSysFfi>,
    http_request_ffi: Arc<THttpRequestFfi>,
}

impl<THttpSysFfi, THttpRequestFfi> HttpSysServerCore<THttpSysFfi, THttpRequestFfi>
where
    THttpSysFfi: HttpSysFfi,
    THttpRequestFfi: HttpRequestFfi,
{
    pub(crate) fn new(
        http_sys_ffi: Arc<THttpSysFfi>,
        http_request_ffi: Arc<THttpRequestFfi>,
    ) -> Self {
        HttpSysServerCore {
            http_sys_ffi,
            http_request_ffi,
        }
    }

    // Boilerplate, to allow the same FFI implementation to be easily cloned.
    pub(crate) fn http_sys_ffi(&self) -> Arc<THttpSysFfi> {
        Arc::clone(&self.http_sys_ffi)
    }

    // Boilerplate, to allow the same FFI implementation to be easily cloned.
    pub(crate) fn http_request_ffi(&self) -> Arc<THttpRequestFfi> {
        Arc::clone(&self.http_request_ffi)
    }

    pub(crate) fn start(&self) -> i32 {
        self.http_sys_ffi.http_start()
    }

    pub(crate) fn accept(&self) -> HttpRequestCore<THttpRequestFfi> {
        HttpRequestCore::new(Arc::clone(&self.http_request_ffi))
    }

    pub(crate) fn server_id(&self) -> i32 {
        1234
    }
}

// This is the public API exposed library consumers.
// It is a wrapper that uses real FFI for everything underneath.
pub struct HttpSysServer(HttpSysServerCore<RealHttpSysFfi, RealHttpRequestFfi>);

// TODO: We could perhaps simplify this newtype generation via an attribute macro on the impl block.
// This macro could automatically generate the necessary wrapper functions.
impl HttpSysServer {
    pub fn new() -> Self {
        HttpSysServer(HttpSysServerCore::new(
            Arc::new(RealHttpSysFfi::default()),
            Arc::new(RealHttpRequestFfi::default()),
        ))
    }

    pub fn start(&self) -> i32 {
        self.0.start()
    }

    pub fn accept(&self) -> HttpRequest {
        self.0.accept().into()
    }
}

// Boilerplate transform between core/public types.
impl From<HttpSysServer> for HttpSysServerCore<RealHttpSysFfi, RealHttpRequestFfi> {
    fn from(http_sys_server: HttpSysServer) -> Self {
        http_sys_server.0
    }
}

// Boilerplate transform between core/public types.
impl From<HttpSysServerCore<RealHttpSysFfi, RealHttpRequestFfi>> for HttpSysServer {
    fn from(http_sys_server_core: HttpSysServerCore<RealHttpSysFfi, RealHttpRequestFfi>) -> Self {
        HttpSysServer(http_sys_server_core)
    }
}

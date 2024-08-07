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

    pub(crate) fn start(&self) -> i32 {
        self.http_sys_ffi.http_start()
    }

    pub(crate) fn accept(&self) -> HttpRequestCore<THttpRequestFfi> {
        HttpRequestCore::new(Arc::clone(&self.http_request_ffi))
    }
}

// This is the public API exposed to Oxidizer partners.
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
        HttpRequest::from_core(self.0.accept())
    }
}

use std::thread;

use crate::{
    ffi::{HttpRequestFfi, HttpSysFfi, RealHttpRequestFfi, RealHttpSysFfi},
    http_request::{HttpRequest, HttpRequestCore},
};

pub(crate) struct HttpSysServerCore<THttpSysFfi, THttpRequestFfi>
where
    THttpSysFfi: HttpSysFfi + 'static,
    THttpRequestFfi: HttpRequestFfi + 'static,
{
    pub(crate) http_sys_ffi: &'static THttpSysFfi,
    pub(crate) http_request_ffi: &'static THttpRequestFfi,
}

impl<THttpSysFfi, THttpRequestFfi> HttpSysServerCore<THttpSysFfi, THttpRequestFfi>
where
    THttpSysFfi: HttpSysFfi + 'static,
    THttpRequestFfi: HttpRequestFfi + 'static,
{
    pub(crate) fn new(
        http_sys_ffi: &'static THttpSysFfi,
        http_request_ffi: &'static THttpRequestFfi,
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
        // Simulate some multithreaded logic.
        thread::spawn({
            let ffi = self.http_request_ffi;

            move || HttpRequestCore::new(ffi)
        })
        .join()
        .unwrap()
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
        HttpSysServer(HttpSysServerCore::new(&RealHttpSysFfi, &RealHttpRequestFfi))
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

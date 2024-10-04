use crate::{
    ffi::{HttpRequestFfi, HttpSysFfi, RealHttpRequestFfi, RealHttpSysFfi},
    http_sys_server::{HttpSysServer, HttpSysServerCore},
};

pub(crate) struct HttpMetricsCore<THttpSysFfi, THttpRequestFfi>
where
    THttpSysFfi: HttpSysFfi + 'static,
    THttpRequestFfi: HttpRequestFfi + 'static,
{
    pub(crate) http_sys_ffi: &'static THttpSysFfi,
    http_server: HttpSysServerCore<THttpSysFfi, THttpRequestFfi>,
}

impl<THttpSysFfi, THttpRequestFfi> HttpMetricsCore<THttpSysFfi, THttpRequestFfi>
where
    THttpSysFfi: HttpSysFfi,
    THttpRequestFfi: HttpRequestFfi,
{
    pub(crate) fn for_server(
        http_sys_server: HttpSysServerCore<THttpSysFfi, THttpRequestFfi>,
    ) -> Self {
        Self {
            http_sys_ffi: http_sys_server.http_sys_ffi,
            http_server: http_sys_server,
        }
    }

    pub(crate) fn get_metrics(&self) -> String {
        self.http_sys_ffi
            .get_metrics_for(self.http_server.server_id())
    }
}

// This is the public API exposed library consumers.
// It is a wrapper that uses real FFI for everything underneath.
pub struct HttpMetrics(HttpMetricsCore<RealHttpSysFfi, RealHttpRequestFfi>);

impl HttpMetrics {
    pub fn for_server(http_server: HttpSysServer) -> Self {
        HttpMetrics(HttpMetricsCore::for_server(http_server.into()))
    }

    pub fn get_metrics(&self) -> String {
        self.0.get_metrics()
    }
}

// Boilerplate transform between core/public types.
impl From<HttpMetrics> for HttpMetricsCore<RealHttpSysFfi, RealHttpRequestFfi> {
    fn from(http_metrics: HttpMetrics) -> Self {
        http_metrics.0
    }
}

// Boilerplate transform between core/public types.
impl From<HttpMetricsCore<RealHttpSysFfi, RealHttpRequestFfi>> for HttpMetrics {
    fn from(http_metrics_core: HttpMetricsCore<RealHttpSysFfi, RealHttpRequestFfi>) -> Self {
        HttpMetrics(http_metrics_core)
    }
}

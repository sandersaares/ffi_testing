#![allow(dead_code)]

mod ffi;
mod http_metrics;
mod http_request;
mod http_sys_server;

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use ffi::{MockHttpRequestFfi, MockHttpSysFfi};
    use http_metrics::{HttpMetrics, HttpMetricsCore};
    use http_sys_server::{HttpSysServer, HttpSysServerCore};

    use super::*;

    #[test]
    fn test_against_real_os() {
        let server = HttpSysServer::new();
        assert_eq!(server.start(), 0);

        let request = server.accept();
        assert_eq!(request.process(), 0);

        let metrics = HttpMetrics::for_server(server);
        assert_eq!(metrics.get_metrics(), "Metrics for server 1234");
    }

    #[test]
    fn test_against_mock_os() {
        static HTTP_SYS_FFI: LazyLock<MockHttpSysFfi> = LazyLock::new(|| {
            let mut mock = MockHttpSysFfi::new();

            mock.expect_http_start().returning(|| 1);
            mock.expect_get_metrics_for()
                .returning(|server_id| format!("Fake metrics for server {}", server_id));

            mock
        });

        static HTTP_REQUEST_FFI: LazyLock<MockHttpRequestFfi> = LazyLock::new(|| {
            let mut mock = MockHttpRequestFfi::new();
            mock.expect_process_request().returning(|| 1);

            mock
        });

        let server = HttpSysServerCore::new(&*HTTP_SYS_FFI, &*HTTP_REQUEST_FFI);
        assert_eq!(server.start(), 1);

        let request = server.accept();
        assert_eq!(request.process(), 1);

        let metrics = HttpMetricsCore::for_server(server);
        assert_eq!(metrics.get_metrics(), "Fake metrics for server 1234");
    }
}

mod ffi;
mod http_request;
mod http_sys_server;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ffi::{MockHttpRequestFfi, MockHttpSysFfi};
    use http_sys_server::{HttpSysServer, HttpSysServerCore};

    use super::*;

    #[test]
    fn test_against_real_os() {
        let server = HttpSysServer::new();
        assert_eq!(server.start(), 0);

        let request = server.accept();
        assert_eq!(request.process(), 0);
    }

    #[test]
    fn test_against_mock_os() {
        let mut http_sys_ffi = MockHttpSysFfi::new();
        http_sys_ffi.expect_http_start().returning(|| 1);

        let mut http_request_ffi = MockHttpRequestFfi::new();
        http_request_ffi.expect_process_request().returning(|| 1);

        let server = HttpSysServerCore::new(Arc::new(http_sys_ffi), Arc::new(http_request_ffi));
        assert_eq!(server.start(), 1);

        let request = server.accept();
        assert_eq!(request.process(), 1);
    }
}

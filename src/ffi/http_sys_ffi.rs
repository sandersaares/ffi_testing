use mockall::automock;

#[automock]
pub(crate) trait HttpSysFfi {
    fn http_start(&self) -> i32;
    fn get_metrics_for(&self, server_id: i32) -> String;
}

#[derive(Default)]
pub(crate) struct RealHttpSysFfi {}

impl HttpSysFfi for RealHttpSysFfi {
    fn http_start(&self) -> i32 {
        0 // TODO: Call the real OS function via windows-sys
    }

    fn get_metrics_for(&self, server_id: i32) -> String {
        format!("Metrics for server {}", server_id) // TODO: Call the real OS function via windows-sys
    }
}

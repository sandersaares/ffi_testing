use mockall::automock;

#[automock]
pub(crate) trait HttpRequestFfi: Send + Sync {
    fn process_request(&self) -> i32;
}

pub(crate) struct RealHttpRequestFfi;

impl HttpRequestFfi for RealHttpRequestFfi {
    fn process_request(&self) -> i32 {
        0 // TODO: Call the real OS function via windows-sys
    }
}

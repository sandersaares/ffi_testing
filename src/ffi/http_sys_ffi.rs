use mockall::automock;

#[automock]
pub(crate) trait HttpSysFfi {
    fn http_start(&self) -> i32;
}

#[derive(Default)]
pub(crate) struct RealHttpSysFfi {}

impl HttpSysFfi for RealHttpSysFfi {
    fn http_start(&self) -> i32 {
        0 // TODO: Call the real OS function via windows-sys
    }
}

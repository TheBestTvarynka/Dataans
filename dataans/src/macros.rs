macro_rules! try_exec {
    ($data:expr, $msg:expr, $toaster:expr) => {
        match $data {
            Ok(data) => data,
            Err(err) => {
                error!("{:?}", err);
                $toaster.toast(
                    leptoaster::ToastBuilder::new(&format!("{}: {}", $msg, err))
                        .with_level(leptoaster::ToastLevel::Error)
                        .with_position(leptoaster::ToastPosition::BottomRight)
                        .with_expiry(Some(5000)),
                );
                return;
            }
        }
    };
}

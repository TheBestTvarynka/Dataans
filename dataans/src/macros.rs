macro_rules! try_exec {
    ($data:expr, $msg:expr, $toaster:expr) => {
        match $data {
            Ok(data) => data,
            Err(err) => {
                error!("{:?}", err);
                $toaster.error(&format!("{}: {:?}", $msg, err));
                return;
            }
        }
    };
}

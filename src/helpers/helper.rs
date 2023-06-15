#[macro_export]
macro_rules! map_err_return {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{:?}", err);
                return None;
            }
        }
    };
}

#[macro_export]
macro_rules! map_none_return {
    ($option:expr, $err_msg:expr, $($arg:expr),*) => {
        match $option {
            Some(value) => value,
            None => {
                eprintln!($err_msg, $($arg),*);
                return None;
            }
        }
    };
}

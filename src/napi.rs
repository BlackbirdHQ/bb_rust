use std::fmt::Display;

use napi::bindgen_prelude as Napi;

trait AnyhowExt<T> {
    fn to_napi_err(self) -> Napi::Result<T>;
}

impl<T, E> AnyhowExt<T> for anyhow::Result<T, E>
where
    E: Display,
{
    fn to_napi_err(self) -> Napi::Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Napi::Error::new(
                napi::Status::GenericFailure,
                format!("{:#}", &e),
            )),
        }
    }
}

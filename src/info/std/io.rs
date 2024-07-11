use std::io;
use crate::PayloadInfo;

impl PayloadInfo for io::Error {
    const TYPE: &'static str = "ioError";
}

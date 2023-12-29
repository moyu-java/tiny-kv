use failure::Fail;
use std::io;
use std::string::FromUtf8Error;

/// tiny-kv 自定义错误类型
#[derive(Fail, Debug)]
pub enum KvError {
    /// IO 错误
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// key 不存在错误
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// 未知命令类型错误
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,

    /// Key or value is invalid UTF-8 sequence
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),

    /// Sled error
    #[fail(display = "sled error: {}", _0)]
    Sled(#[cause] sled::Error),

    /// Error with a string message
    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<io::Error> for KvError {
    fn from(err: io::Error) -> KvError {
        KvError::Io(err)
    }
}

impl From<FromUtf8Error> for KvError {
    fn from(err: FromUtf8Error) -> KvError {
        KvError::Utf8(err)
    }
}


impl From<sled::Error> for KvError {
    fn from(err: sled::Error) -> KvError {
        KvError::Sled(err)
    }
}

pub type Result<T> = std::result::Result<T, KvError>;
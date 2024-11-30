// Automatically generated rust module for 'constants.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{BytesReader, Result, MessageInfo, MessageRead, MessageWrite};
use core::convert::{TryFrom, TryInto};
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Success = 0,
    InvalidSecScheme = 1,
    InvalidProto = 2,
    TooManySessions = 3,
    InvalidArgument = 4,
    InternalError = 5,
    CryptoError = 6,
    InvalidSession = 7,
}

impl Default for Status {
    fn default() -> Self {
        Status::Success
    }
}

impl From<i32> for Status {
    fn from(i: i32) -> Self {
        match i {
            0 => Status::Success,
            1 => Status::InvalidSecScheme,
            2 => Status::InvalidProto,
            3 => Status::TooManySessions,
            4 => Status::InvalidArgument,
            5 => Status::InternalError,
            6 => Status::CryptoError,
            7 => Status::InvalidSession,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Self {
        match s {
            "Success" => Status::Success,
            "InvalidSecScheme" => Status::InvalidSecScheme,
            "InvalidProto" => Status::InvalidProto,
            "TooManySessions" => Status::TooManySessions,
            "InvalidArgument" => Status::InvalidArgument,
            "InternalError" => Status::InternalError,
            "CryptoError" => Status::CryptoError,
            "InvalidSession" => Status::InvalidSession,
            _ => Self::default(),
        }
    }
}


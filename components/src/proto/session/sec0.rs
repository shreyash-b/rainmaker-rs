// Automatically generated rust module for 'sec0.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use core::convert::{TryFrom, TryInto};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sec0MsgType {
    S0_Session_Command = 0,
    S0_Session_Response = 1,
}

impl Default for Sec0MsgType {
    fn default() -> Self {
        Sec0MsgType::S0_Session_Command
    }
}

impl From<i32> for Sec0MsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => Sec0MsgType::S0_Session_Command,
            1 => Sec0MsgType::S0_Session_Response,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Sec0MsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "S0_Session_Command" => Sec0MsgType::S0_Session_Command,
            "S0_Session_Response" => Sec0MsgType::S0_Session_Response,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct S0SessionCmd { }

impl<'a> MessageRead<'a> for S0SessionCmd {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for S0SessionCmd { }


            impl TryFrom<&[u8]> for S0SessionCmd {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(S0SessionCmd::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct S0SessionResp {
    pub status: constants::Status,
}

impl<'a> MessageRead<'a> for S0SessionResp {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for S0SessionResp {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for S0SessionResp {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(S0SessionResp::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sec0Payload {
    pub msg: Sec0MsgType,
    pub payload: mod_Sec0Payload::OneOfpayload,
}

impl<'a> MessageRead<'a> for Sec0Payload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msg = r.read_enum(bytes)?,
                Ok(162) => msg.payload = mod_Sec0Payload::OneOfpayload::sc(r.read_message::<S0SessionCmd>(bytes)?),
                Ok(170) => msg.payload = mod_Sec0Payload::OneOfpayload::sr(r.read_message::<S0SessionResp>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Sec0Payload {
    fn get_size(&self) -> usize {
        0
        + if self.msg == sec0::Sec0MsgType::S0_Session_Command { 0 } else { 1 + sizeof_varint(*(&self.msg) as u64) }
        + match self.payload {
            mod_Sec0Payload::OneOfpayload::sc(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec0Payload::OneOfpayload::sr(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec0Payload::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg != sec0::Sec0MsgType::S0_Session_Command { w.write_with_tag(8, |w| w.write_enum(*&self.msg as i32))?; }
        match self.payload {            mod_Sec0Payload::OneOfpayload::sc(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            mod_Sec0Payload::OneOfpayload::sr(ref m) => { w.write_with_tag(170, |w| w.write_message(m))? },
            mod_Sec0Payload::OneOfpayload::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for Sec0Payload {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Sec0Payload::from_reader(&mut reader, &buf)?)
                }
            }
            
pub mod mod_Sec0Payload {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload {
    sc(S0SessionCmd),
    sr(S0SessionResp),
    None,
}

impl Default for OneOfpayload {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}


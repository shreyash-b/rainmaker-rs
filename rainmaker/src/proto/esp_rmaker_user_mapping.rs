// Automatically generated rust module for 'esp_rmaker_user_mapping.proto' file

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
pub enum RMakerConfigStatus {
    Success = 0,
    InvalidParam = 1,
    InvalidState = 2,
}

impl Default for RMakerConfigStatus {
    fn default() -> Self {
        RMakerConfigStatus::Success
    }
}

impl From<i32> for RMakerConfigStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => RMakerConfigStatus::Success,
            1 => RMakerConfigStatus::InvalidParam,
            2 => RMakerConfigStatus::InvalidState,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RMakerConfigStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "Success" => RMakerConfigStatus::Success,
            "InvalidParam" => RMakerConfigStatus::InvalidParam,
            "InvalidState" => RMakerConfigStatus::InvalidState,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RMakerConfigMsgType {
    TypeCmdSetUserMapping = 0,
    TypeRespSetUserMapping = 1,
}

impl Default for RMakerConfigMsgType {
    fn default() -> Self {
        RMakerConfigMsgType::TypeCmdSetUserMapping
    }
}

impl From<i32> for RMakerConfigMsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => RMakerConfigMsgType::TypeCmdSetUserMapping,
            1 => RMakerConfigMsgType::TypeRespSetUserMapping,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RMakerConfigMsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "TypeCmdSetUserMapping" => RMakerConfigMsgType::TypeCmdSetUserMapping,
            "TypeRespSetUserMapping" => RMakerConfigMsgType::TypeRespSetUserMapping,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdSetUserMapping {
    pub UserID: String,
    pub SecretKey: String,
}

impl<'a> MessageRead<'a> for CmdSetUserMapping {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.UserID = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.SecretKey = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CmdSetUserMapping {
    fn get_size(&self) -> usize {
        0
        + if self.UserID == String::default() { 0 } else { 1 + sizeof_len((&self.UserID).len()) }
        + if self.SecretKey == String::default() { 0 } else { 1 + sizeof_len((&self.SecretKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.UserID != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.UserID))?; }
        if self.SecretKey != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.SecretKey))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for CmdSetUserMapping {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdSetUserMapping::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespSetUserMapping {
    pub Status: RMakerConfigStatus,
    pub NodeId: String,
}

impl<'a> MessageRead<'a> for RespSetUserMapping {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.Status = r.read_enum(bytes)?,
                Ok(18) => msg.NodeId = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RespSetUserMapping {
    fn get_size(&self) -> usize {
        0
        + if self.Status == RMakerConfigStatus::Success { 0 } else { 1 + sizeof_varint(*(&self.Status) as u64) }
        + if self.NodeId == String::default() { 0 } else { 1 + sizeof_len((&self.NodeId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.Status != RMakerConfigStatus::Success { w.write_with_tag(8, |w| w.write_enum(*&self.Status as i32))?; }
        if self.NodeId != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.NodeId))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespSetUserMapping {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespSetUserMapping::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RMakerConfigPayload {
    pub msg: RMakerConfigMsgType,
    pub payload: mod_RMakerConfigPayload::OneOfpayload,
}

impl<'a> MessageRead<'a> for RMakerConfigPayload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msg = r.read_enum(bytes)?,
                Ok(82) => msg.payload = mod_RMakerConfigPayload::OneOfpayload::cmd_set_user_mapping(r.read_message::<CmdSetUserMapping>(bytes)?),
                Ok(90) => msg.payload = mod_RMakerConfigPayload::OneOfpayload::resp_set_user_mapping(r.read_message::<RespSetUserMapping>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RMakerConfigPayload {
    fn get_size(&self) -> usize {
        0
        + if self.msg == RMakerConfigMsgType::TypeCmdSetUserMapping { 0 } else { 1 + sizeof_varint(*(&self.msg) as u64) }
        + match self.payload {
            mod_RMakerConfigPayload::OneOfpayload::cmd_set_user_mapping(ref m) => 1 + sizeof_len((m).get_size()),
            mod_RMakerConfigPayload::OneOfpayload::resp_set_user_mapping(ref m) => 1 + sizeof_len((m).get_size()),
            mod_RMakerConfigPayload::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg != RMakerConfigMsgType::TypeCmdSetUserMapping { w.write_with_tag(8, |w| w.write_enum(*&self.msg as i32))?; }
        match self.payload {            mod_RMakerConfigPayload::OneOfpayload::cmd_set_user_mapping(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_RMakerConfigPayload::OneOfpayload::resp_set_user_mapping(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_RMakerConfigPayload::OneOfpayload::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for RMakerConfigPayload {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RMakerConfigPayload::from_reader(&mut reader, &buf)?)
                }
            }
            
pub mod mod_RMakerConfigPayload {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload {
    cmd_set_user_mapping(CmdSetUserMapping),
    resp_set_user_mapping(RespSetUserMapping),
    None,
}

impl Default for OneOfpayload {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}


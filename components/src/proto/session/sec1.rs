// Automatically generated rust module for 'sec1.proto' file

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
pub enum Sec1MsgType {
    Session_Command0 = 0,
    Session_Response0 = 1,
    Session_Command1 = 2,
    Session_Response1 = 3,
}

impl Default for Sec1MsgType {
    fn default() -> Self {
        Sec1MsgType::Session_Command0
    }
}

impl From<i32> for Sec1MsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => Sec1MsgType::Session_Command0,
            1 => Sec1MsgType::Session_Response0,
            2 => Sec1MsgType::Session_Command1,
            3 => Sec1MsgType::Session_Response1,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Sec1MsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "Session_Command0" => Sec1MsgType::Session_Command0,
            "Session_Response0" => Sec1MsgType::Session_Response0,
            "Session_Command1" => Sec1MsgType::Session_Command1,
            "Session_Response1" => Sec1MsgType::Session_Response1,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SessionCmd1 {
    pub client_verify_data: Vec<u8>,
}

impl<'a> MessageRead<'a> for SessionCmd1 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.client_verify_data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SessionCmd1 {
    fn get_size(&self) -> usize {
        0
        + if self.client_verify_data.is_empty() { 0 } else { 1 + sizeof_len((&self.client_verify_data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.client_verify_data.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.client_verify_data))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for SessionCmd1 {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(SessionCmd1::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SessionResp1 {
    pub status: constants::Status,
    pub device_verify_data: Vec<u8>,
}

impl<'a> MessageRead<'a> for SessionResp1 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(26) => msg.device_verify_data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SessionResp1 {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.device_verify_data.is_empty() { 0 } else { 1 + sizeof_len((&self.device_verify_data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        if !self.device_verify_data.is_empty() { w.write_with_tag(26, |w| w.write_bytes(&**&self.device_verify_data))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for SessionResp1 {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(SessionResp1::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SessionCmd0 {
    pub client_pubkey: Vec<u8>,
}

impl<'a> MessageRead<'a> for SessionCmd0 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.client_pubkey = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SessionCmd0 {
    fn get_size(&self) -> usize {
        0
        + if self.client_pubkey.is_empty() { 0 } else { 1 + sizeof_len((&self.client_pubkey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.client_pubkey.is_empty() { w.write_with_tag(10, |w| w.write_bytes(&**&self.client_pubkey))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for SessionCmd0 {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(SessionCmd0::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SessionResp0 {
    pub status: constants::Status,
    pub device_pubkey: Vec<u8>,
    pub device_random: Vec<u8>,
}

impl<'a> MessageRead<'a> for SessionResp0 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(18) => msg.device_pubkey = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.device_random = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SessionResp0 {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.device_pubkey.is_empty() { 0 } else { 1 + sizeof_len((&self.device_pubkey).len()) }
        + if self.device_random.is_empty() { 0 } else { 1 + sizeof_len((&self.device_random).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        if !self.device_pubkey.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.device_pubkey))?; }
        if !self.device_random.is_empty() { w.write_with_tag(26, |w| w.write_bytes(&**&self.device_random))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for SessionResp0 {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(SessionResp0::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sec1Payload {
    pub msg: Sec1MsgType,
    pub payload: mod_Sec1Payload::OneOfpayload,
}

impl<'a> MessageRead<'a> for Sec1Payload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msg = r.read_enum(bytes)?,
                Ok(162) => msg.payload = mod_Sec1Payload::OneOfpayload::sc0(r.read_message::<SessionCmd0>(bytes)?),
                Ok(170) => msg.payload = mod_Sec1Payload::OneOfpayload::sr0(r.read_message::<SessionResp0>(bytes)?),
                Ok(178) => msg.payload = mod_Sec1Payload::OneOfpayload::sc1(r.read_message::<SessionCmd1>(bytes)?),
                Ok(186) => msg.payload = mod_Sec1Payload::OneOfpayload::sr1(r.read_message::<SessionResp1>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Sec1Payload {
    fn get_size(&self) -> usize {
        0
        + if self.msg == sec1::Sec1MsgType::Session_Command0 { 0 } else { 1 + sizeof_varint(*(&self.msg) as u64) }
        + match self.payload {
            mod_Sec1Payload::OneOfpayload::sc0(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec1Payload::OneOfpayload::sr0(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec1Payload::OneOfpayload::sc1(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec1Payload::OneOfpayload::sr1(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec1Payload::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg != sec1::Sec1MsgType::Session_Command0 { w.write_with_tag(8, |w| w.write_enum(*&self.msg as i32))?; }
        match self.payload {            mod_Sec1Payload::OneOfpayload::sc0(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            mod_Sec1Payload::OneOfpayload::sr0(ref m) => { w.write_with_tag(170, |w| w.write_message(m))? },
            mod_Sec1Payload::OneOfpayload::sc1(ref m) => { w.write_with_tag(178, |w| w.write_message(m))? },
            mod_Sec1Payload::OneOfpayload::sr1(ref m) => { w.write_with_tag(186, |w| w.write_message(m))? },
            mod_Sec1Payload::OneOfpayload::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for Sec1Payload {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Sec1Payload::from_reader(&mut reader, &buf)?)
                }
            }
            
pub mod mod_Sec1Payload {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload {
    sc0(SessionCmd0),
    sr0(SessionResp0),
    sc1(SessionCmd1),
    sr1(SessionResp1),
    None,
}

impl Default for OneOfpayload {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}


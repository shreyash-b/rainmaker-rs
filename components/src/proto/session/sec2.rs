// Automatically generated rust module for 'sec2.proto' file

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
pub enum Sec2MsgType {
    S2Session_Command0 = 0,
    S2Session_Response0 = 1,
    S2Session_Command1 = 2,
    S2Session_Response1 = 3,
}

impl Default for Sec2MsgType {
    fn default() -> Self {
        Sec2MsgType::S2Session_Command0
    }
}

impl From<i32> for Sec2MsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => Sec2MsgType::S2Session_Command0,
            1 => Sec2MsgType::S2Session_Response0,
            2 => Sec2MsgType::S2Session_Command1,
            3 => Sec2MsgType::S2Session_Response1,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Sec2MsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "S2Session_Command0" => Sec2MsgType::S2Session_Command0,
            "S2Session_Response0" => Sec2MsgType::S2Session_Response0,
            "S2Session_Command1" => Sec2MsgType::S2Session_Command1,
            "S2Session_Response1" => Sec2MsgType::S2Session_Response1,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct S2SessionCmd0 {
    pub client_username: Vec<u8>,
    pub client_pubkey: Vec<u8>,
}

impl<'a> MessageRead<'a> for S2SessionCmd0 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.client_username = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.client_pubkey = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for S2SessionCmd0 {
    fn get_size(&self) -> usize {
        0
        + if self.client_username.is_empty() { 0 } else { 1 + sizeof_len((&self.client_username).len()) }
        + if self.client_pubkey.is_empty() { 0 } else { 1 + sizeof_len((&self.client_pubkey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.client_username.is_empty() { w.write_with_tag(10, |w| w.write_bytes(&**&self.client_username))?; }
        if !self.client_pubkey.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.client_pubkey))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for S2SessionCmd0 {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(S2SessionCmd0::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct S2SessionResp0 {
    pub status: constants::Status,
    pub device_pubkey: Vec<u8>,
    pub device_salt: Vec<u8>,
}

impl<'a> MessageRead<'a> for S2SessionResp0 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(18) => msg.device_pubkey = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.device_salt = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for S2SessionResp0 {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.device_pubkey.is_empty() { 0 } else { 1 + sizeof_len((&self.device_pubkey).len()) }
        + if self.device_salt.is_empty() { 0 } else { 1 + sizeof_len((&self.device_salt).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        if !self.device_pubkey.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.device_pubkey))?; }
        if !self.device_salt.is_empty() { w.write_with_tag(26, |w| w.write_bytes(&**&self.device_salt))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for S2SessionResp0 {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(S2SessionResp0::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct S2SessionCmd1 {
    pub client_proof: Vec<u8>,
}

impl<'a> MessageRead<'a> for S2SessionCmd1 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.client_proof = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for S2SessionCmd1 {
    fn get_size(&self) -> usize {
        0
        + if self.client_proof.is_empty() { 0 } else { 1 + sizeof_len((&self.client_proof).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.client_proof.is_empty() { w.write_with_tag(10, |w| w.write_bytes(&**&self.client_proof))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for S2SessionCmd1 {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(S2SessionCmd1::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct S2SessionResp1 {
    pub status: constants::Status,
    pub device_proof: Vec<u8>,
    pub device_nonce: Vec<u8>,
}

impl<'a> MessageRead<'a> for S2SessionResp1 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(18) => msg.device_proof = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.device_nonce = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for S2SessionResp1 {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.device_proof.is_empty() { 0 } else { 1 + sizeof_len((&self.device_proof).len()) }
        + if self.device_nonce.is_empty() { 0 } else { 1 + sizeof_len((&self.device_nonce).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        if !self.device_proof.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.device_proof))?; }
        if !self.device_nonce.is_empty() { w.write_with_tag(26, |w| w.write_bytes(&**&self.device_nonce))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for S2SessionResp1 {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(S2SessionResp1::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sec2Payload {
    pub msg: Sec2MsgType,
    pub payload: mod_Sec2Payload::OneOfpayload,
}

impl<'a> MessageRead<'a> for Sec2Payload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msg = r.read_enum(bytes)?,
                Ok(162) => msg.payload = mod_Sec2Payload::OneOfpayload::sc0(r.read_message::<S2SessionCmd0>(bytes)?),
                Ok(170) => msg.payload = mod_Sec2Payload::OneOfpayload::sr0(r.read_message::<S2SessionResp0>(bytes)?),
                Ok(178) => msg.payload = mod_Sec2Payload::OneOfpayload::sc1(r.read_message::<S2SessionCmd1>(bytes)?),
                Ok(186) => msg.payload = mod_Sec2Payload::OneOfpayload::sr1(r.read_message::<S2SessionResp1>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Sec2Payload {
    fn get_size(&self) -> usize {
        0
        + if self.msg == sec2::Sec2MsgType::S2Session_Command0 { 0 } else { 1 + sizeof_varint(*(&self.msg) as u64) }
        + match self.payload {
            mod_Sec2Payload::OneOfpayload::sc0(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec2Payload::OneOfpayload::sr0(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec2Payload::OneOfpayload::sc1(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec2Payload::OneOfpayload::sr1(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Sec2Payload::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg != sec2::Sec2MsgType::S2Session_Command0 { w.write_with_tag(8, |w| w.write_enum(*&self.msg as i32))?; }
        match self.payload {            mod_Sec2Payload::OneOfpayload::sc0(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            mod_Sec2Payload::OneOfpayload::sr0(ref m) => { w.write_with_tag(170, |w| w.write_message(m))? },
            mod_Sec2Payload::OneOfpayload::sc1(ref m) => { w.write_with_tag(178, |w| w.write_message(m))? },
            mod_Sec2Payload::OneOfpayload::sr1(ref m) => { w.write_with_tag(186, |w| w.write_message(m))? },
            mod_Sec2Payload::OneOfpayload::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for Sec2Payload {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Sec2Payload::from_reader(&mut reader, &buf)?)
                }
            }
            
pub mod mod_Sec2Payload {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload {
    sc0(S2SessionCmd0),
    sr0(S2SessionResp0),
    sc1(S2SessionCmd1),
    sr1(S2SessionResp1),
    None,
}

impl Default for OneOfpayload {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}


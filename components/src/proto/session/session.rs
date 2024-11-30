// Automatically generated rust module for 'session.proto' file

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
pub enum SecSchemeVersion {
    SecScheme0 = 0,
    SecScheme1 = 1,
    SecScheme2 = 2,
}

impl Default for SecSchemeVersion {
    fn default() -> Self {
        SecSchemeVersion::SecScheme0
    }
}

impl From<i32> for SecSchemeVersion {
    fn from(i: i32) -> Self {
        match i {
            0 => SecSchemeVersion::SecScheme0,
            1 => SecSchemeVersion::SecScheme1,
            2 => SecSchemeVersion::SecScheme2,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SecSchemeVersion {
    fn from(s: &'a str) -> Self {
        match s {
            "SecScheme0" => SecSchemeVersion::SecScheme0,
            "SecScheme1" => SecSchemeVersion::SecScheme1,
            "SecScheme2" => SecSchemeVersion::SecScheme2,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SessionData {
    pub sec_ver: SecSchemeVersion,
    pub proto: mod_SessionData::OneOfproto,
}

impl<'a> MessageRead<'a> for SessionData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.sec_ver = r.read_enum(bytes)?,
                Ok(82) => msg.proto = mod_SessionData::OneOfproto::sec0(r.read_message::<sec0::Sec0Payload>(bytes)?),
                Ok(90) => msg.proto = mod_SessionData::OneOfproto::sec1(r.read_message::<sec1::Sec1Payload>(bytes)?),
                Ok(98) => msg.proto = mod_SessionData::OneOfproto::sec2(r.read_message::<sec2::Sec2Payload>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SessionData {
    fn get_size(&self) -> usize {
        0
        + if self.sec_ver == session::SecSchemeVersion::SecScheme0 { 0 } else { 1 + sizeof_varint(*(&self.sec_ver) as u64) }
        + match self.proto {
            mod_SessionData::OneOfproto::sec0(ref m) => 1 + sizeof_len((m).get_size()),
            mod_SessionData::OneOfproto::sec1(ref m) => 1 + sizeof_len((m).get_size()),
            mod_SessionData::OneOfproto::sec2(ref m) => 1 + sizeof_len((m).get_size()),
            mod_SessionData::OneOfproto::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.sec_ver != session::SecSchemeVersion::SecScheme0 { w.write_with_tag(16, |w| w.write_enum(*&self.sec_ver as i32))?; }
        match self.proto {            mod_SessionData::OneOfproto::sec0(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_SessionData::OneOfproto::sec1(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_SessionData::OneOfproto::sec2(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_SessionData::OneOfproto::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for SessionData {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(SessionData::from_reader(&mut reader, &buf)?)
                }
            }
            
pub mod mod_SessionData {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfproto {
    sec0(sec0::Sec0Payload),
    sec1(sec1::Sec1Payload),
    sec2(sec2::Sec2Payload),
    None,
}

impl Default for OneOfproto {
    fn default() -> Self {
        OneOfproto::None
    }
}

}


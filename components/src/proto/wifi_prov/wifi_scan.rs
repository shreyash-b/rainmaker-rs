// Automatically generated rust module for 'wifi_scan.proto' file

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
pub enum WiFiScanMsgType {
    TypeCmdScanStart = 0,
    TypeRespScanStart = 1,
    TypeCmdScanStatus = 2,
    TypeRespScanStatus = 3,
    TypeCmdScanResult = 4,
    TypeRespScanResult = 5,
}

impl Default for WiFiScanMsgType {
    fn default() -> Self {
        WiFiScanMsgType::TypeCmdScanStart
    }
}

impl From<i32> for WiFiScanMsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => WiFiScanMsgType::TypeCmdScanStart,
            1 => WiFiScanMsgType::TypeRespScanStart,
            2 => WiFiScanMsgType::TypeCmdScanStatus,
            3 => WiFiScanMsgType::TypeRespScanStatus,
            4 => WiFiScanMsgType::TypeCmdScanResult,
            5 => WiFiScanMsgType::TypeRespScanResult,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for WiFiScanMsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "TypeCmdScanStart" => WiFiScanMsgType::TypeCmdScanStart,
            "TypeRespScanStart" => WiFiScanMsgType::TypeRespScanStart,
            "TypeCmdScanStatus" => WiFiScanMsgType::TypeCmdScanStatus,
            "TypeRespScanStatus" => WiFiScanMsgType::TypeRespScanStatus,
            "TypeCmdScanResult" => WiFiScanMsgType::TypeCmdScanResult,
            "TypeRespScanResult" => WiFiScanMsgType::TypeRespScanResult,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdScanStart {
    pub blocking: bool,
    pub passive: bool,
    pub group_channels: u32,
    pub period_ms: u32,
}

impl<'a> MessageRead<'a> for CmdScanStart {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.blocking = r.read_bool(bytes)?,
                Ok(16) => msg.passive = r.read_bool(bytes)?,
                Ok(24) => msg.group_channels = r.read_uint32(bytes)?,
                Ok(32) => msg.period_ms = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CmdScanStart {
    fn get_size(&self) -> usize {
        0
        + if self.blocking == false { 0 } else { 1 + sizeof_varint(*(&self.blocking) as u64) }
        + if self.passive == false { 0 } else { 1 + sizeof_varint(*(&self.passive) as u64) }
        + if self.group_channels == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.group_channels) as u64) }
        + if self.period_ms == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.period_ms) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.blocking != false { w.write_with_tag(8, |w| w.write_bool(*&self.blocking))?; }
        if self.passive != false { w.write_with_tag(16, |w| w.write_bool(*&self.passive))?; }
        if self.group_channels != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.group_channels))?; }
        if self.period_ms != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.period_ms))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for CmdScanStart {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdScanStart::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespScanStart { }

impl<'a> MessageRead<'a> for RespScanStart {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RespScanStart { }


            impl TryFrom<&[u8]> for RespScanStart {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespScanStart::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdScanStatus { }

impl<'a> MessageRead<'a> for CmdScanStatus {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for CmdScanStatus { }


            impl TryFrom<&[u8]> for CmdScanStatus {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdScanStatus::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespScanStatus {
    pub scan_finished: bool,
    pub result_count: u32,
}

impl<'a> MessageRead<'a> for RespScanStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.scan_finished = r.read_bool(bytes)?,
                Ok(16) => msg.result_count = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RespScanStatus {
    fn get_size(&self) -> usize {
        0
        + if self.scan_finished == false { 0 } else { 1 + sizeof_varint(*(&self.scan_finished) as u64) }
        + if self.result_count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.result_count) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.scan_finished != false { w.write_with_tag(8, |w| w.write_bool(*&self.scan_finished))?; }
        if self.result_count != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.result_count))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespScanStatus {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespScanStatus::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdScanResult {
    pub start_index: u32,
    pub count: u32,
}

impl<'a> MessageRead<'a> for CmdScanResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.start_index = r.read_uint32(bytes)?,
                Ok(16) => msg.count = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CmdScanResult {
    fn get_size(&self) -> usize {
        0
        + if self.start_index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.start_index) as u64) }
        + if self.count == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.count) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.start_index != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.start_index))?; }
        if self.count != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.count))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for CmdScanResult {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdScanResult::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct WiFiScanResult {
    pub ssid: Vec<u8>,
    pub channel: u32,
    pub rssi: i32,
    pub bssid: Vec<u8>,
    pub auth: wifi_constants::WifiAuthMode,
}

impl<'a> MessageRead<'a> for WiFiScanResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ssid = r.read_bytes(bytes)?.to_owned(),
                Ok(16) => msg.channel = r.read_uint32(bytes)?,
                Ok(24) => msg.rssi = r.read_int32(bytes)?,
                Ok(34) => msg.bssid = r.read_bytes(bytes)?.to_owned(),
                Ok(40) => msg.auth = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for WiFiScanResult {
    fn get_size(&self) -> usize {
        0
        + if self.ssid.is_empty() { 0 } else { 1 + sizeof_len((&self.ssid).len()) }
        + if self.channel == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.channel) as u64) }
        + if self.rssi == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.rssi) as u64) }
        + if self.bssid.is_empty() { 0 } else { 1 + sizeof_len((&self.bssid).len()) }
        + if self.auth == wifi_constants::WifiAuthMode::Open { 0 } else { 1 + sizeof_varint(*(&self.auth) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.ssid.is_empty() { w.write_with_tag(10, |w| w.write_bytes(&**&self.ssid))?; }
        if self.channel != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.channel))?; }
        if self.rssi != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.rssi))?; }
        if !self.bssid.is_empty() { w.write_with_tag(34, |w| w.write_bytes(&**&self.bssid))?; }
        if self.auth != wifi_constants::WifiAuthMode::Open { w.write_with_tag(40, |w| w.write_enum(*&self.auth as i32))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for WiFiScanResult {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(WiFiScanResult::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespScanResult {
    pub entries: Vec<WiFiScanResult>,
}

impl<'a> MessageRead<'a> for RespScanResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.entries.push(r.read_message::<WiFiScanResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RespScanResult {
    fn get_size(&self) -> usize {
        0
        + self.entries.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.entries { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespScanResult {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespScanResult::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct WiFiScanPayload {
    pub msg: WiFiScanMsgType,
    pub status: constants::Status,
    pub payload: mod_WiFiScanPayload::OneOfpayload,
}

impl<'a> MessageRead<'a> for WiFiScanPayload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msg = r.read_enum(bytes)?,
                Ok(16) => msg.status = r.read_enum(bytes)?,
                Ok(82) => msg.payload = mod_WiFiScanPayload::OneOfpayload::cmd_scan_start(r.read_message::<CmdScanStart>(bytes)?),
                Ok(90) => msg.payload = mod_WiFiScanPayload::OneOfpayload::resp_scan_start(r.read_message::<RespScanStart>(bytes)?),
                Ok(98) => msg.payload = mod_WiFiScanPayload::OneOfpayload::cmd_scan_status(r.read_message::<CmdScanStatus>(bytes)?),
                Ok(106) => msg.payload = mod_WiFiScanPayload::OneOfpayload::resp_scan_status(r.read_message::<RespScanStatus>(bytes)?),
                Ok(114) => msg.payload = mod_WiFiScanPayload::OneOfpayload::cmd_scan_result(r.read_message::<CmdScanResult>(bytes)?),
                Ok(122) => msg.payload = mod_WiFiScanPayload::OneOfpayload::resp_scan_result(r.read_message::<RespScanResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for WiFiScanPayload {
    fn get_size(&self) -> usize {
        0
        + if self.msg == wifi_scan::WiFiScanMsgType::TypeCmdScanStart { 0 } else { 1 + sizeof_varint(*(&self.msg) as u64) }
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + match self.payload {
            mod_WiFiScanPayload::OneOfpayload::cmd_scan_start(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiScanPayload::OneOfpayload::resp_scan_start(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiScanPayload::OneOfpayload::cmd_scan_status(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiScanPayload::OneOfpayload::resp_scan_status(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiScanPayload::OneOfpayload::cmd_scan_result(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiScanPayload::OneOfpayload::resp_scan_result(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiScanPayload::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg != wifi_scan::WiFiScanMsgType::TypeCmdScanStart { w.write_with_tag(8, |w| w.write_enum(*&self.msg as i32))?; }
        if self.status != constants::Status::Success { w.write_with_tag(16, |w| w.write_enum(*&self.status as i32))?; }
        match self.payload {            mod_WiFiScanPayload::OneOfpayload::cmd_scan_start(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_WiFiScanPayload::OneOfpayload::resp_scan_start(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_WiFiScanPayload::OneOfpayload::cmd_scan_status(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_WiFiScanPayload::OneOfpayload::resp_scan_status(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_WiFiScanPayload::OneOfpayload::cmd_scan_result(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_WiFiScanPayload::OneOfpayload::resp_scan_result(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            mod_WiFiScanPayload::OneOfpayload::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for WiFiScanPayload {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(WiFiScanPayload::from_reader(&mut reader, &buf)?)
                }
            }
            
pub mod mod_WiFiScanPayload {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload {
    cmd_scan_start(CmdScanStart),
    resp_scan_start(RespScanStart),
    cmd_scan_status(CmdScanStatus),
    resp_scan_status(RespScanStatus),
    cmd_scan_result(CmdScanResult),
    resp_scan_result(RespScanResult),
    None,
}

impl Default for OneOfpayload {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}


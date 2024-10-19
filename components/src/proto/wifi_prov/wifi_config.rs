// Automatically generated rust module for 'wifi_config.proto' file

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
pub enum WiFiConfigMsgType {
    TypeCmdGetStatus = 0,
    TypeRespGetStatus = 1,
    TypeCmdSetConfig = 2,
    TypeRespSetConfig = 3,
    TypeCmdApplyConfig = 4,
    TypeRespApplyConfig = 5,
}

impl Default for WiFiConfigMsgType {
    fn default() -> Self {
        WiFiConfigMsgType::TypeCmdGetStatus
    }
}

impl From<i32> for WiFiConfigMsgType {
    fn from(i: i32) -> Self {
        match i {
            0 => WiFiConfigMsgType::TypeCmdGetStatus,
            1 => WiFiConfigMsgType::TypeRespGetStatus,
            2 => WiFiConfigMsgType::TypeCmdSetConfig,
            3 => WiFiConfigMsgType::TypeRespSetConfig,
            4 => WiFiConfigMsgType::TypeCmdApplyConfig,
            5 => WiFiConfigMsgType::TypeRespApplyConfig,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for WiFiConfigMsgType {
    fn from(s: &'a str) -> Self {
        match s {
            "TypeCmdGetStatus" => WiFiConfigMsgType::TypeCmdGetStatus,
            "TypeRespGetStatus" => WiFiConfigMsgType::TypeRespGetStatus,
            "TypeCmdSetConfig" => WiFiConfigMsgType::TypeCmdSetConfig,
            "TypeRespSetConfig" => WiFiConfigMsgType::TypeRespSetConfig,
            "TypeCmdApplyConfig" => WiFiConfigMsgType::TypeCmdApplyConfig,
            "TypeRespApplyConfig" => WiFiConfigMsgType::TypeRespApplyConfig,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdGetStatus { }

impl<'a> MessageRead<'a> for CmdGetStatus {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for CmdGetStatus { }


            impl TryFrom<&[u8]> for CmdGetStatus {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdGetStatus::from_reader(&mut reader, &buf)?)
                }
            }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespGetStatus {
    pub status: constants::Status,
    pub sta_state: wifi_constants::WifiStationState,
    pub state: mod_RespGetStatus::OneOfstate,
}

impl<'a> MessageRead<'a> for RespGetStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(16) => msg.sta_state = r.read_enum(bytes)?,
                Ok(80) => msg.state = mod_RespGetStatus::OneOfstate::fail_reason(r.read_enum(bytes)?),
                Ok(90) => msg.state = mod_RespGetStatus::OneOfstate::connected(r.read_message::<wifi_constants::WifiConnectedState>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RespGetStatus {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.sta_state == wifi_constants::WifiStationState::Connected { 0 } else { 1 + sizeof_varint(*(&self.sta_state) as u64) }
        + match self.state {
            mod_RespGetStatus::OneOfstate::fail_reason(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_RespGetStatus::OneOfstate::connected(ref m) => 1 + sizeof_len((m).get_size()),
            mod_RespGetStatus::OneOfstate::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        if self.sta_state != wifi_constants::WifiStationState::Connected { w.write_with_tag(16, |w| w.write_enum(*&self.sta_state as i32))?; }
        match self.state {            mod_RespGetStatus::OneOfstate::fail_reason(ref m) => { w.write_with_tag(80, |w| w.write_enum(*m as i32))? },
            mod_RespGetStatus::OneOfstate::connected(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_RespGetStatus::OneOfstate::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespGetStatus {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespGetStatus::from_reader(&mut reader, &buf)?)
                }
            }

pub mod mod_RespGetStatus {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfstate {
    fail_reason(wifi_constants::WifiConnectFailedReason),
    connected(wifi_constants::WifiConnectedState),
    None,
}

impl Default for OneOfstate {
    fn default() -> Self {
        OneOfstate::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdSetConfig {
    pub ssid: Vec<u8>,
    pub passphrase: Vec<u8>,
    pub bssid: Vec<u8>,
    pub channel: i32,
}

impl<'a> MessageRead<'a> for CmdSetConfig {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ssid = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.passphrase = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.bssid = r.read_bytes(bytes)?.to_owned(),
                Ok(32) => msg.channel = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CmdSetConfig {
    fn get_size(&self) -> usize {
        0
        + if self.ssid.is_empty() { 0 } else { 1 + sizeof_len((&self.ssid).len()) }
        + if self.passphrase.is_empty() { 0 } else { 1 + sizeof_len((&self.passphrase).len()) }
        + if self.bssid.is_empty() { 0 } else { 1 + sizeof_len((&self.bssid).len()) }
        + if self.channel == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.channel) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.ssid.is_empty() { w.write_with_tag(10, |w| w.write_bytes(&**&self.ssid))?; }
        if !self.passphrase.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.passphrase))?; }
        if !self.bssid.is_empty() { w.write_with_tag(26, |w| w.write_bytes(&**&self.bssid))?; }
        if self.channel != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.channel))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for CmdSetConfig {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdSetConfig::from_reader(&mut reader, &buf)?)
                }
            }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespSetConfig {
    pub status: constants::Status,
}

impl<'a> MessageRead<'a> for RespSetConfig {
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

impl MessageWrite for RespSetConfig {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespSetConfig {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespSetConfig::from_reader(&mut reader, &buf)?)
                }
            }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CmdApplyConfig { }

impl<'a> MessageRead<'a> for CmdApplyConfig {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for CmdApplyConfig { }


            impl TryFrom<&[u8]> for CmdApplyConfig {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(CmdApplyConfig::from_reader(&mut reader, &buf)?)
                }
            }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RespApplyConfig {
    pub status: constants::Status,
}

impl<'a> MessageRead<'a> for RespApplyConfig {
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

impl MessageWrite for RespApplyConfig {
    fn get_size(&self) -> usize {
        0
        + if self.status == constants::Status::Success { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != constants::Status::Success { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RespApplyConfig {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RespApplyConfig::from_reader(&mut reader, &buf)?)
                }
            }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct WiFiConfigPayload {
    pub msg: WiFiConfigMsgType,
    pub payload: mod_WiFiConfigPayload::OneOfpayload,
}

impl<'a> MessageRead<'a> for WiFiConfigPayload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.msg = r.read_enum(bytes)?,
                Ok(82) => msg.payload = mod_WiFiConfigPayload::OneOfpayload::cmd_get_status(r.read_message::<CmdGetStatus>(bytes)?),
                Ok(90) => msg.payload = mod_WiFiConfigPayload::OneOfpayload::resp_get_status(r.read_message::<RespGetStatus>(bytes)?),
                Ok(98) => msg.payload = mod_WiFiConfigPayload::OneOfpayload::cmd_set_config(r.read_message::<CmdSetConfig>(bytes)?),
                Ok(106) => msg.payload = mod_WiFiConfigPayload::OneOfpayload::resp_set_config(r.read_message::<RespSetConfig>(bytes)?),
                Ok(114) => msg.payload = mod_WiFiConfigPayload::OneOfpayload::cmd_apply_config(r.read_message::<CmdApplyConfig>(bytes)?),
                Ok(122) => msg.payload = mod_WiFiConfigPayload::OneOfpayload::resp_apply_config(r.read_message::<RespApplyConfig>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for WiFiConfigPayload {
    fn get_size(&self) -> usize {
        0
        + if self.msg == wifi_config::WiFiConfigMsgType::TypeCmdGetStatus { 0 } else { 1 + sizeof_varint(*(&self.msg) as u64) }
        + match self.payload {
            mod_WiFiConfigPayload::OneOfpayload::cmd_get_status(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiConfigPayload::OneOfpayload::resp_get_status(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiConfigPayload::OneOfpayload::cmd_set_config(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiConfigPayload::OneOfpayload::resp_set_config(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiConfigPayload::OneOfpayload::cmd_apply_config(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiConfigPayload::OneOfpayload::resp_apply_config(ref m) => 1 + sizeof_len((m).get_size()),
            mod_WiFiConfigPayload::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg != wifi_config::WiFiConfigMsgType::TypeCmdGetStatus { w.write_with_tag(8, |w| w.write_enum(*&self.msg as i32))?; }
        match self.payload {            mod_WiFiConfigPayload::OneOfpayload::cmd_get_status(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_WiFiConfigPayload::OneOfpayload::resp_get_status(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_WiFiConfigPayload::OneOfpayload::cmd_set_config(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_WiFiConfigPayload::OneOfpayload::resp_set_config(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_WiFiConfigPayload::OneOfpayload::cmd_apply_config(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_WiFiConfigPayload::OneOfpayload::resp_apply_config(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            mod_WiFiConfigPayload::OneOfpayload::None => {},
    }        Ok(())
    }
}


            impl TryFrom<&[u8]> for WiFiConfigPayload {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(WiFiConfigPayload::from_reader(&mut reader, &buf)?)
                }
            }

pub mod mod_WiFiConfigPayload {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload {
    cmd_get_status(CmdGetStatus),
    resp_get_status(RespGetStatus),
    cmd_set_config(CmdSetConfig),
    resp_set_config(RespSetConfig),
    cmd_apply_config(CmdApplyConfig),
    resp_apply_config(RespApplyConfig),
    None,
}

impl Default for OneOfpayload {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}

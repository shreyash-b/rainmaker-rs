// Automatically generated rust module for 'wifi_constants.proto' file

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
pub enum WifiStationState {
    Connected = 0,
    Connecting = 1,
    Disconnected = 2,
    ConnectionFailed = 3,
}

impl Default for WifiStationState {
    fn default() -> Self {
        WifiStationState::Connected
    }
}

impl From<i32> for WifiStationState {
    fn from(i: i32) -> Self {
        match i {
            0 => WifiStationState::Connected,
            1 => WifiStationState::Connecting,
            2 => WifiStationState::Disconnected,
            3 => WifiStationState::ConnectionFailed,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for WifiStationState {
    fn from(s: &'a str) -> Self {
        match s {
            "Connected" => WifiStationState::Connected,
            "Connecting" => WifiStationState::Connecting,
            "Disconnected" => WifiStationState::Disconnected,
            "ConnectionFailed" => WifiStationState::ConnectionFailed,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WifiConnectFailedReason {
    AuthError = 0,
    NetworkNotFound = 1,
}

impl Default for WifiConnectFailedReason {
    fn default() -> Self {
        WifiConnectFailedReason::AuthError
    }
}

impl From<i32> for WifiConnectFailedReason {
    fn from(i: i32) -> Self {
        match i {
            0 => WifiConnectFailedReason::AuthError,
            1 => WifiConnectFailedReason::NetworkNotFound,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for WifiConnectFailedReason {
    fn from(s: &'a str) -> Self {
        match s {
            "AuthError" => WifiConnectFailedReason::AuthError,
            "NetworkNotFound" => WifiConnectFailedReason::NetworkNotFound,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WifiAuthMode {
    Open = 0,
    WEP = 1,
    WPA_PSK = 2,
    WPA2_PSK = 3,
    WPA_WPA2_PSK = 4,
    WPA2_ENTERPRISE = 5,
    WPA3_PSK = 6,
    WPA2_WPA3_PSK = 7,
}

impl Default for WifiAuthMode {
    fn default() -> Self {
        WifiAuthMode::Open
    }
}

impl From<i32> for WifiAuthMode {
    fn from(i: i32) -> Self {
        match i {
            0 => WifiAuthMode::Open,
            1 => WifiAuthMode::WEP,
            2 => WifiAuthMode::WPA_PSK,
            3 => WifiAuthMode::WPA2_PSK,
            4 => WifiAuthMode::WPA_WPA2_PSK,
            5 => WifiAuthMode::WPA2_ENTERPRISE,
            6 => WifiAuthMode::WPA3_PSK,
            7 => WifiAuthMode::WPA2_WPA3_PSK,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for WifiAuthMode {
    fn from(s: &'a str) -> Self {
        match s {
            "Open" => WifiAuthMode::Open,
            "WEP" => WifiAuthMode::WEP,
            "WPA_PSK" => WifiAuthMode::WPA_PSK,
            "WPA2_PSK" => WifiAuthMode::WPA2_PSK,
            "WPA_WPA2_PSK" => WifiAuthMode::WPA_WPA2_PSK,
            "WPA2_ENTERPRISE" => WifiAuthMode::WPA2_ENTERPRISE,
            "WPA3_PSK" => WifiAuthMode::WPA3_PSK,
            "WPA2_WPA3_PSK" => WifiAuthMode::WPA2_WPA3_PSK,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct WifiConnectedState {
    pub ip4_addr: String,
    pub auth_mode: WifiAuthMode,
    pub ssid: Vec<u8>,
    pub bssid: Vec<u8>,
    pub channel: i32,
}

impl<'a> MessageRead<'a> for WifiConnectedState {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ip4_addr = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.auth_mode = r.read_enum(bytes)?,
                Ok(26) => msg.ssid = r.read_bytes(bytes)?.to_owned(),
                Ok(34) => msg.bssid = r.read_bytes(bytes)?.to_owned(),
                Ok(40) => msg.channel = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for WifiConnectedState {
    fn get_size(&self) -> usize {
        0
        + if self.ip4_addr == String::default() { 0 } else { 1 + sizeof_len((&self.ip4_addr).len()) }
        + if self.auth_mode == wifi_constants::WifiAuthMode::Open { 0 } else { 1 + sizeof_varint(*(&self.auth_mode) as u64) }
        + if self.ssid.is_empty() { 0 } else { 1 + sizeof_len((&self.ssid).len()) }
        + if self.bssid.is_empty() { 0 } else { 1 + sizeof_len((&self.bssid).len()) }
        + if self.channel == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.channel) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ip4_addr != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.ip4_addr))?; }
        if self.auth_mode != wifi_constants::WifiAuthMode::Open { w.write_with_tag(16, |w| w.write_enum(*&self.auth_mode as i32))?; }
        if !self.ssid.is_empty() { w.write_with_tag(26, |w| w.write_bytes(&**&self.ssid))?; }
        if !self.bssid.is_empty() { w.write_with_tag(34, |w| w.write_bytes(&**&self.bssid))?; }
        if self.channel != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.channel))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for WifiConnectedState {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(WifiConnectedState::from_reader(&mut reader, &buf)?)
                }
            }
            

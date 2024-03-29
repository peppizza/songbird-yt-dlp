use super::context_data::*;
use crate::ConnectionInfo;
use discortp::{rtcp::Rtcp, rtp::Rtp};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InternalConnect {
    pub info: ConnectionInfo,
    pub ssrc: u32,
}

#[derive(Debug)]
pub struct InternalDisconnect {
    pub kind: DisconnectKind,
    pub reason: Option<DisconnectReason>,
    pub info: ConnectionInfo,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InternalSpeakingUpdate {
    pub ssrc: u32,
    pub speaking: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalVoicePacket {
    pub audio: Option<Vec<i16>>,
    pub packet: Rtp,
    pub payload_offset: usize,
    pub payload_end_pad: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalRtcpPacket {
    pub packet: Rtcp,
    pub payload_offset: usize,
    pub payload_end_pad: usize,
}

impl<'a> From<&'a InternalConnect> for ConnectData<'a> {
    fn from(val: &'a InternalConnect) -> Self {
        Self {
            channel_id: val.info.channel_id,
            guild_id: val.info.guild_id,
            session_id: &val.info.session_id,
            server: &val.info.endpoint,
            ssrc: val.ssrc,
        }
    }
}

impl<'a> From<&'a InternalDisconnect> for DisconnectData<'a> {
    fn from(val: &'a InternalDisconnect) -> Self {
        Self {
            kind: val.kind,
            reason: val.reason,
            channel_id: val.info.channel_id,
            guild_id: val.info.guild_id,
            session_id: &val.info.session_id,
        }
    }
}

impl<'a> From<&'a InternalSpeakingUpdate> for SpeakingUpdateData {
    fn from(val: &'a InternalSpeakingUpdate) -> Self {
        Self {
            speaking: val.speaking,
            ssrc: val.ssrc,
        }
    }
}

impl<'a> From<&'a InternalVoicePacket> for VoiceData<'a> {
    fn from(val: &'a InternalVoicePacket) -> Self {
        Self {
            audio: &val.audio,
            packet: &val.packet,
            payload_offset: val.payload_offset,
            payload_end_pad: val.payload_end_pad,
        }
    }
}

impl<'a> From<&'a InternalRtcpPacket> for RtcpData<'a> {
    fn from(val: &'a InternalRtcpPacket) -> Self {
        Self {
            packet: &val.packet,
            payload_offset: val.payload_offset,
            payload_end_pad: val.payload_end_pad,
        }
    }
}

use std::borrow::Cow;
use crate::timestamp::Timestamp;
use crate::event::DataId;

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(PartialEq, Debug, Readable, Writable)]
pub enum Request {
    StartStreaming,
    TriggerMemoryDump,
    Ping
}

#[derive(PartialEq, Debug, Readable, Writable)]
pub enum Response< 'a > {
    Start( BroadcastHeader ),
    Data( Cow< 'a, [u8] > ),
    FinishedInitialStreaming,
    Pong
}

#[derive(PartialEq, Debug, Readable, Writable)]
pub struct BroadcastHeader {
    pub id: DataId,
    pub initial_timestamp: Timestamp,
    pub timestamp: Timestamp,
    pub wall_clock_secs: u64,
    pub wall_clock_nsecs: u64,
    pub pid: u32,
    pub cmdline: Vec< u8 >,
    pub executable: Vec< u8 >,
    pub arch: String,
    pub listener_port: u16,
    pub protocol_version: u32
}

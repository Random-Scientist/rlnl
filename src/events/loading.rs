use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};

use crate::{
    events::register_event_type,
    types::{BinaryWriterString, StringCode},
};

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct PlayerIDAndName {
    pub player_id: i32,
    pub name: BinaryWriterString,
    pub display_name: BinaryWriterString,
}

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct PlayerIDsAndNames {
    #[byteserde(replace(players.len()))]
    num_players: u8,
    #[byteserde(deplete(num_players as usize))]
    pub players: Vec<PlayerIDAndName>,
}
register_event_type! { PlayerIDsAndNames, PlayerIDs }

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct PlayerIDs {
    #[byteserde(replace(players.len()))]
    num_ids: i32,
    #[byteserde(deplete(usize::try_from(num_ids).unwrap()))]
    pub players: Vec<i32>,
}
register_event_type! { PlayerIDs, HostAIs }

register_event_type! { StringCode, WarnPlayer }

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct LoadingProgress {
    pub user_name: BinaryWriterString,
    pub progress: f32,
}

register_event_type! { LoadingProgress, BroadcastLoadingProgress }
// EacMessage/EacMessageDependency probably unneeded, skipped

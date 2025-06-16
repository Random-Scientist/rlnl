use byteserde::prelude::{ByteDeserializeSlice, ByteSerializeHeap};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};

use crate::{
    events::register_event_type,
    types::{
        BinaryWriterString, Byte3, CubeStatus, IngameStatId, TargetType, UnityCompressedVec3,
        VoteType,
    },
    util::bitflag_bits,
};
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SetFinalGameScore {
    pub player_id: u8,
    pub score: i32,
}
register_event_type! { SetFinalGameScore, SetFinalGameScore }

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct UpdateGameStats {
    pub player_id: u8,
    pub stat_id: IngameStatId,
    pub amount: u32,
    pub score: u32,
    pub delta_score: u32,
}
register_event_type! { UpdateGameStats, UpdateGameStats }

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct UpdateVotingAfterBattle {
    pub player_name: BinaryWriterString,
    pub amount: i32,
    pub vote_type: VoteType,
}
register_event_type! { UpdateVotingAfterBattle, UpdateVotingAfterBattle }

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct Kill {
    pub killee_player_id: i32,
    pub killer_player_id: u8,
}
register_event_type! { Kill, MachineDestroyedConfirmed }

bitflag_bits! {
    #[derive(Debug, Clone, Copy)]
    #[repr(transparent)]
    pub struct PackedInputData: u16 bits: {
        JUMP: 1,
        CROUCH: 2,
        PULSE_AR: 3,
        TOGGLE_LIGHTS: 4,
        HORIZONTAL_POSITIVE: 5,
        HORIZONTAL_NEGATIVE: 6,
        VERTICAL_POSITIVE: 7,
        VERTICAL_NEGATIVE: 8,
        STRAFE_LEFT: 9,
        STRAFE_RIGHT: 10,
    }
}

impl ByteDeserializeSlice<PackedInputData> for PackedInputData {
    fn byte_deserialize(
        des: &mut byteserde::prelude::ByteDeserializerSlice,
    ) -> byteserde::error::Result<PackedInputData> {
        Ok(PackedInputData::from_bits_retain(u16::from_le_bytes(
            *des.deserialize_bytes_array_ref()?,
        )))
    }
}
impl ByteSerializeHeap for PackedInputData {
    fn byte_serialize_heap(
        &self,
        ser: &mut byteserde::prelude::ByteSerializerHeap,
    ) -> byteserde::error::Result<()> {
        ser.serialize_bytes_slice(&self.bits().to_le_bytes())?;
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct PlayerIdAndInputData {
    pub player_id: u8,
    pub input_data: PackedInputData,
}

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct MultiPlayerInputChanged {
    #[byteserde(replace(changes.len()))]
    pub num_players: u8,
    #[byteserde(deplete(usize::from(num_players)))]
    pub changes: Vec<PlayerIdAndInputData>,
}
register_event_type! { MultiPlayerInputChanged, OnServerReceivedInputChange }

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct DestroyCubesFull {
    pub shooting_machine_id: i16,
    pub hit_machine_id: i16,
    pub item_category: i16,
    pub stack_count: u8,
    pub target_type: TargetType,
    pub weapon_damage: i32,
    pub hit_effect_offset: UnityCompressedVec3<768>,
    pub hit_effect_normal: UnityCompressedVec3<255>,
    #[byteserde(replace(hit_cubes.len()))]
    pub num_hit_cubes: u16,
    #[byteserde(deplete(usize::from(num_hit_cubes)))]
    pub hit_cubes: Vec<CubeStatus>,
    pub timestamp: f32,
}
register_event_type! { DestroyCubesFull, DestroyCubesFull }

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct DestroyCubeEffectOnly {
    pub shooting_machine_id: i16,
    pub hit_machine_id: i16,
    pub item_category: i16,
    pub stack_count: u8,
    pub target_type: TargetType,
    pub hit_effect_offset: UnityCompressedVec3<768>,
    pub hit_effect_normal: UnityCompressedVec3<255>,
    pub hit_cube: Byte3,
}

register_event_type! { DestroyCubeEffectOnly, DestroyCubeEffectOnly }

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct DestroyCubeNoEffect {
    pub shooting_machine_id: i16,
    pub hit_machine_id: i16,
    pub target_type: TargetType,
    #[byteserde(replace(hit_cubes.len()))]
    pub num_hits: u16,
    #[byteserde(deplete(usize::from(num_hits)))]
    pub hit_cubes: Vec<CubeStatus>,
}
register_event_type! { DestroyCubeNoEffect, DestroyCubeNoEffect }

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct WeaponFireEffect {
    launch_position: UnityCompressedVec3<768>,
    direction: UnityCompressedVec3<768>,
    shooting_machine_id: i16,
    shooting_player_id: u8,
    weapon_grid_key: Byte3,
}
register_event_type! { WeaponFireEffect, FireWeaponEffect }

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct FireMiss {
    shooting_machine_id: u16,
    hit_point: UnityCompressedVec3<32768>,
    hit_normal: UnityCompressedVec3<255>,
    time_stamp: f32,
    packed_hit_hitself: u8,
    target_type: TargetType,
    item_category: i32,
    item_size: i32,
}
register_event_type! { FireMiss, FireMiss }
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct FireMissEntry {
    hit_point: UnityCompressedVec3<32768>,
    hit_normal: UnityCompressedVec3<255>,
    packed_hit_hitself: u8,
    target_type: TargetType,
}
#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct MultipleFireMisses {
    #[byteserde(replace(hits.len()))]
    num_hits: u8,
    shooting_machine_id: i16,
    #[byteserde(deplete(usize::from(num_hits)))]
    hits: Vec<FireMissEntry>,
    timestamp: f32,
    item_category: i16,
    item_size: i16,
}

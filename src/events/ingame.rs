use byteserde::prelude::{ByteDeserializeSlice, ByteSerializeHeap};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};

use crate::{
    types::{
        BinaryWriterString, Byte3, ByteFloat, CompressedVec3, CubeStatus, DVec3, GameEndReason,
        HitCubeInfo, IngameStatId, ItemDescriptor, PingType, PosQuatPair, SQuat, SVec3, TargetType,
        VoteType,
    },
    util::bitflag_bits,
};
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SetFinalGameScore {
    pub player_id: u8,
    pub score: i32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct UpdateGameStats {
    pub player_id: u8,
    pub stat_id: IngameStatId,
    pub amount: u32,
    pub score: u32,
    pub delta_score: u32,
}

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct UpdateVotingAfterBattle {
    pub player_name: BinaryWriterString,
    pub amount: i32,
    pub vote_type: VoteType,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct Kill {
    pub killee_player_id: i32,
    pub killer_player_id: u8,
}

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

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct DestroyCubesFull {
    pub shooting_machine_id: i16,
    pub hit_machine_id: i16,
    pub item_category: i16,
    pub item_size: i16,
    pub stack_count: u8,
    pub target_type: TargetType,
    pub weapon_damage: i32,
    pub hit_effect_offset: CompressedVec3<768>,
    pub hit_effect_normal: CompressedVec3<255>,
    #[byteserde(replace(hit_cubes.len()))]
    pub num_hit_cubes: u16,
    #[byteserde(deplete(usize::from(num_hit_cubes)))]
    pub hit_cubes: Vec<CubeStatus>,
    pub timestamp: f32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct DestroyCubeEffectOnly {
    pub shooting_machine_id: i16,
    pub hit_machine_id: i16,
    pub item_category: i16,
    pub stack_count: u8,
    pub target_type: TargetType,
    pub hit_effect_offset: CompressedVec3<768>,
    pub hit_effect_normal: CompressedVec3<255>,
    pub hit_cube: Byte3,
}

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

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct WeaponFireEffect {
    pub launch_position: CompressedVec3<768>,
    pub direction: CompressedVec3<768>,
    pub shooting_machine_id: i16,
    pub shooting_player_id: u8,
    pub weapon_grid_key: Byte3,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct FireMiss {
    pub shooting_machine_id: u16,
    pub hit_point: CompressedVec3<32768>,
    pub hit_normal: CompressedVec3<255>,
    pub time_stamp: f32,
    pub packed_hit_hitself: u8,
    pub target_type: TargetType,
    // can't use ItemDescriptor because they decided not to pack them for this event...
    pub item_category: i32,
    pub item_size: i32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct FireMissEntry {
    pub hit_point: CompressedVec3<32768>,
    pub hit_normal: CompressedVec3<255>,
    pub packed_hit_hitself: u8,
    pub target_type: TargetType,
}

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct MultipleFireMisses {
    #[byteserde(replace(hits.len()))]
    pub num_hits: u8,
    pub shooting_machine_id: i16,
    #[byteserde(deplete(usize::from(num_hits)))]
    pub hits: Vec<FireMissEntry>,
    pub timestamp: f32,
    pub desc: ItemDescriptor,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GameStart {
    // bool
    pub is_reconnecting: u8,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GameEnd {
    pub reason: GameEndReason,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct RequestPing {
    pub player_id: u8,
    pub requester: u8,
    pub timestamp: f32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct PlayerId {
    pub player: u8,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct MapPing {
    pub sender: i32,
    pub team_id: i32,
    pub ty: PingType,
    pub pos: DVec3,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct LockOnNotifier {
    pub firing_player_id: u8,
    pub target_player_id: u8,
    pub lock_stage: u8,
    pub locked_cube_pos: Byte3,
    pub item_category: i32,
    pub item_size: i32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct ShieldModuleEvent {
    pub pos: PosQuatPair,
    pub firing_player_id: u8,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct TeleportActivateEffect {
    //bool
    pub activate: u8,
    pub player_id: u8,
    pub module_index: u8,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SpawnEmpLocator {
    pub pos: CompressedVec3<768>,
    pub range: f32,
    pub countdown: f32,
    pub stun_duration: f32,
    pub owner_id: u8,
    pub owner_machine_id: i16,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct NetworkStunnedMachineEffect {
    pub machine_id: i32,
    //bool
    pub is_stunned: u8,
    pub owner_id: i32,
}

#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct Taunt {
    pub machine_id: i32,
    pub taunt_id: BinaryWriterString,
    pub relative_position: SVec3,
    pub relative_orientation: SQuat,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct CosmeticAction {
    pub owner_machine_id: i32,
    pub cosmetic_action_data_index: i32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SelectWeapon {
    pub machine_id: u8,
    pub item_category: u32,
    pub item_size: u32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct HealAllyEntry {
    pub cube_info: HitCubeInfo,
    pub type_performing_healing: TargetType,
}
#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct HealAllyCubes {
    pub healed_machine: i16,
    pub shooting_machine: i16,
    pub shooting_player_id: u8,
    pub item_size: i32,
    pub hit_effect_offset: CompressedVec3<768>,
    pub hit_effect_normal: CompressedVec3<255>,
    pub time_stamp: f32,
    #[byteserde(replace(hit_cubes.len()))]
    pub num_healed_cubes: u16,
    #[byteserde(deplete(num_healed_cubes as usize))]
    pub hit_cubes: Vec<HealAllyEntry>,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct RespawnTime {
    pub owner: u8,
    pub waiting_time: i16,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GameLoseWin {
    pub winning_team: u8,
    pub end_reason: GameEndReason,
}
#[derive(Debug, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct CurrentSurrenderVotes {
    players_on_team: i32,
    #[byteserde(replace(votes.len()))]
    num_votes: i32,
    //vec<bool>
    #[byteserde(deplete(usize::try_from(num_votes).expect("negative number of votes in CurrentSurrenderVotes")))]
    votes: Vec<u8>,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SurrenderDeclined {
    surrendering_player_id: i32,
    game_time_elapsed: f32,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SurrenderTimes {
    player_cooldown_seconds: i32,
    team_cooldown_seconds: i32,
    surrender_timeout_seconds: i32,
    initial_surrender_timeout_seconds: i32,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct TeamBaseBoolean {
    team: u8,
    //bool
    value: u8,
}
#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct TeamBaseState {
    base_team_or_mining_point_index: u8,
    current_progress: ByteFloat<4>,
    max_progress: ByteFloat<4>,
}

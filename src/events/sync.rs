use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};

use crate::{
    events::register_event_type,
    types::{
        CapturePoint, CubeState, EqualizerState, HitCubeInfo, IngamePlayerStats, PosQuatPair,
        TargetType,
    },
};
const NUM_CAPTURE_POINTS: usize = 3;

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct UpdateGameModeSettings {
    pub respawn_heal_duration: f32,
    pub respawn_full_heal_duration: f32,
}
register_event_type! { UpdateGameModeSettings, GameModeSettings }

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GetTeamBase {
    pub base_1: PosQuatPair,
    pub base_2: PosQuatPair,
    pub protonium_cube_health: i32,
}
register_event_type! { GetTeamBase, TeamBase }

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GetCapturePoints {
    pub points: [CapturePoint; NUM_CAPTURE_POINTS],
}
register_event_type! { GetCapturePoints, RegisterCapturePoints }

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GetEqualizer {
    pub pos: PosQuatPair,
    pub total_health: i32,
}
register_event_type! { GetEqualizer, RegisterEqualizer }

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct FusionShieldState {
    pub team_id: i8,
    pub full_power: u8,
}
register_event_type! { FusionShieldState, SetShieldState }

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GameTime(f32);
register_event_type! { GameTime, CurrentGameTime }

#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct HealedCubes {
    pub healed_machine: u16,
    pub type_performing_healing: TargetType,
    pub target_type: TargetType,
    #[byteserde(replace(hit_cubes.len()))]
    num_healed_cubes: u16,
    #[byteserde(deplete(num_healed_cubes as usize))]
    pub hit_cubes: Vec<HitCubeInfo>,
}
register_event_type! { HealedCubes, SyncTeamBaseCubes }

#[derive(Debug, Default, Copy, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct EqualizerNotification {
    pub notification: EqualizerState,
    pub team_id: i16,
    pub time: i16,
    pub max_health: i32,
    pub health: i32,
}
register_event_type! { EqualizerNotification, EqualizerNotification }

#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct InitialiseGameStats {
    #[byteserde(replace(stats.len()))]
    num_players: u8,
    #[byteserde(deplete(num_players as usize))]
    pub stats: Vec<IngamePlayerStats>,
}
register_event_type! { InitialiseGameStats, InitialiseGameStats }

#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SpawnPoint {
    pos: PosQuatPair,
    pub owner: u8,
}
register_event_type! { SpawnPoint, FreeSpawnPoint }

#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SyncMachineCubes {
    pub machine_id: u16,
    #[byteserde(replace(events.len()))]
    num_cubes: i32,
    #[byteserde(deplete(usize::try_from(num_cubes).expect("negative amount of cubes in SyncMachineCubes")))]
    pub events: Vec<CubeState>,
}
register_event_type! { SyncMachineCubes, SyncMachineCubes }

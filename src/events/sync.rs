use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};

use crate::types::{
    CapturePoint, CubeState, EqualizerState, GameModeSettings, IngamePlayerStats, PosQuatPair,
};
const NUM_CAPTURE_POINTS: usize = 3;

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct UpdateGameModeSettings {
    pub respawn_heal_duration: f32,
    pub respawn_full_heal_duration: f32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GetTeamBase {
    pub base_1: PosQuatPair,
    pub base_2: PosQuatPair,
    pub protonium_cube_health: i32,
}

#[derive(Debug, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GetCapturePoints {
    pub points: [CapturePoint; NUM_CAPTURE_POINTS],
}

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GetEqualizer {
    pub pos: PosQuatPair,
    pub total_health: i32,
}

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct FusionShieldState {
    pub team_id: i8,
    pub full_power: u8,
}

#[derive(Debug, Default, Copy, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct EqualizerNotification {
    pub notification: EqualizerState,
    pub team_id: i16,
    pub time: i16,
    pub max_health: i32,
    pub health: i32,
}
#[derive(Debug, Default, Copy, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct UpdateTeamDeathmatchSettings {
    pub settings: GameModeSettings,
}
#[derive(Debug, Default, Copy, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct TeamScore {
    pub team_id: i32,
    pub score: i32,
}
#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct UpdateTeamDeathMatch {
    #[byteserde(replace(team_scores.len()))]
    pub num_teams: i32,
    #[byteserde(deplete(usize::try_from(num_teams).expect("negative number of teams in UpdateTeamDeathMatch")))]
    pub team_scores: Vec<TeamScore>,
    //bool
    pub time_expired: u8,
}

#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct InitialiseGameStats {
    #[byteserde(replace(stats.len()))]
    pub num_players: u8,
    #[byteserde(deplete(num_players as usize))]
    pub stats: Vec<IngamePlayerStats>,
}

#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SpawnPoint {
    pub pos: PosQuatPair,
    pub owner: u8,
}

#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SyncMachineCubes {
    pub machine_id: u16,
    #[byteserde(replace(events.len()))]
    pub num_cubes: i32,
    #[byteserde(deplete(usize::try_from(num_cubes).expect("negative amount of cubes in SyncMachineCubes")))]
    pub events: Vec<CubeState>,
}

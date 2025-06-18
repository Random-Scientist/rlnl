use crate::{
    events::{
        ingame::{
            CosmeticAction, CurrentSurrenderVotes, DestroyCubeEffectOnly, DestroyCubeNoEffect,
            DestroyCubesFull, FireMiss, GameEnd, GameLoseWin, GameStart, HealAllyCubes, Kill,
            LockOnNotifier, MapPing, MultiPlayerInputChanged, MultipleFireMisses,
            NetworkStunnedMachineEffect, PlayerId, RequestPing, RespawnTime, SelectWeapon,
            SetFinalGameScore, SpawnEmpLocator, SurrenderDeclined, SurrenderTimes, Taunt,
            TeamBaseBoolean, TeamBaseState, TeleportActivateEffect, UpdateGameStats,
            UpdateVotingAfterBattle, WeaponFireEffect,
        },
        loading::{LoadingProgress, PlayerIDs, PlayerIDsAndNames},
        sync::{
            EqualizerNotification, FusionShieldState, GetCapturePoints, GetEqualizer, GetTeamBase,
            InitialiseGameStats, SpawnPoint, SyncMachineCubes, UpdateGameModeSettings,
            UpdateTeamDeathMatch, UpdateTeamDeathmatchSettings,
        },
    },
    types::{HitCubeInfo, StringCode, TargetType},
};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};
use sealed::sealed;
pub mod ingame;
pub mod loading;
pub mod sync;

pub trait GameMode {}
pub enum BattleArena {}
impl GameMode for BattleArena {}
pub enum TeamDeathMatch {}
impl GameMode for TeamDeathMatch {}
pub enum Elimination {}
impl GameMode for Elimination {}
pub enum Pit {}
impl GameMode for Pit {}

pub struct ConstEvent<const N: u8>;

#[sealed(pub(crate))]
pub trait TypedEvent<Mode: GameMode> {
    type Data;
    //fn parse(&self, data: &[u8]) -> Self::Data;
}

macro_rules! register_event_types {
    (dollar = $_:tt; $( , )? ) => {};
    {
        #[dollar = $_:tt]
        $( $rest:tt )*
    } => {
        crate::events::register_event_types! {dollar = $_; $( $rest )* }
    };
    (
        dollar = $_:tt;
        , $( $rest:tt )*
    ) => {
        crate::events::register_event_types! { dollar = $_; $( $rest )* }
    };
    (
        dollar = $_:tt;
        All: {
            $(
                { $ty:ty: $( $event:ident ),+ $( , )? }
            ),+
            $( , )?
        } $( $rest:tt )*
    ) => {
        $(
            $(
                #[::sealed::sealed]
            impl<M: crate::events::GameMode> crate::events::TypedEvent<M> for crate::events::ConstEvent<{ crate::event_code::NetworkEvent::$event as u8 }> {
                type Data = $ty;
            }
            )+
        )+
        crate::events::register_event_types! { dollar = $_; $( $rest )* }
    };
    (
        dollar = $_:tt;
        $( $bound:ident ),+ : {
            $(
                { $ty:ty: $( $event:ident ),+ $( , )? }
            ),+
            $( , )?
        } $( $rest:tt )*
    ) => {
        macro_rules! inner {
            ($_bound:ident) => {
                $(
                    $(
                        #[::sealed::sealed]
                        impl crate::events::TypedEvent<crate::events::$_bound> for crate::events::ConstEvent<{ crate::event_code::NetworkEvent::$event as u8 }> {
                            type Data = $ty;
                        }
                    )+
                )+
            }
        }
        $(
            inner! { $bound }
        )+
        crate::events::register_event_types! { dollar = $_; $( $rest )* }
    };


    ($ty:ty, $( $event:ident ),+ ) => {
        $(
            #[::sealed::sealed]
            impl<M: crate::events::GameMode> crate::events::TypedEvent<M> for crate::events::ConstEvent<{ crate::event_code::NetworkEvent::$event as u8 }> {
                type Data = $ty;
            }
        )+
    };
}
pub(crate) use register_event_types;

register_event_types! {
    #[dollar = $]
    // Sync events
    BattleArena, TeamDeathMatch, Elimination: {
        { GameTime: CurrentGameTime },
    }
    BattleArena, Elimination, Pit: {
        {UpdateGameModeSettings: GameModeSettings }
    }
    TeamDeathMatch: {
        { UpdateTeamDeathmatchSettings: GameModeSettings },
        { UpdateTeamDeathMatch: TeamDeathMatchState },
    }
    BattleArena: {
        { GetTeamBase: TeamBase },
        { GetCapturePoints: RegisterCapturePoints },
        { GetEqualizer: RegisterEqualizer },
        { FusionShieldState: SetShieldState },
        { HealedCubes: SyncTeamBaseCubes },
        { EqualizerNotification: EqualizerNotification },
    }
    All: {
        { InitialiseGameStats: InitialiseGameStats },
        { SpawnPoint: FreeSpawnPoint },
        { SyncMachineCubes: SyncMachineCubes },
        { CommandOnly: BeginSync, EndOfSync },
    }
    // Loading events
    All: {
        { PlayerIDsAndNames: PlayerIDs },
        { PlayerIDs: HostAIs },
        { StringCode: WarnPlayer },
        { LoadingProgress: BroadcastLoadingProgress },
    }
    // Ingame events
    All: {
        { SetFinalGameScore: SetFinalGameScore },
        { UpdateGameStats: UpdateGameStats },
        { UpdateVotingAfterBattle: UpdateVotingAfterBattle },
        { Kill: MachineDestroyedConfirmed },
        { MultiPlayerInputChanged: OnServerReceivedInputChange },
        { DestroyCubesFull: DestroyCubesFull },
        { DestroyCubeEffectOnly: DestroyCubeEffectOnly },
        { DestroyCubeNoEffect: DestroyCubeNoEffect },
        { WeaponFireEffect: FireWeaponEffect },
        { FireMiss: FireMiss },
        { MultipleFireMisses: MultipleFireMisses },
        { GameTime: TimeToGameStart },
        { GameStart: GameStarted },
        { GameEnd: EndGame },
        { RequestPing: GetClientPings, SetClientPing },
        { PlayerId: AlignmentRectifierStarted },
        { MapPing: MapPingEvent },
        { HealedCubes: HealSelfResponse },
        { Kill: ConfirmedKill, ConfirmedAssist },
        { LockOnNotifier: LockOnNotificationBroadcast },
        { TeleportActivateEffect: ActivateTeleportEffect },
        { SpawnEmpLocator: SpawnEmpLocator },
        { NetworkStunnedMachineEffect: SpawnEmpMachineEffect },
        { Taunt: Taunt },
        { CosmeticAction: CosmeticAction },
        { SelectWeapon: BroadcastWeaponSelect },
        { HealAllyCubes: HealAllyResponse },
        {
            PlayerId:
            MakeInvisible,
            MakeVisible,
            EnergyModuleActivated,
            RemoteRadarModuleActivated,
            OnAnotherClientDisconnected,
            OnClientReconnected,
            AcquireRemoteAI,
        },
        { CommandOnly: PlayerQuitRequestComplete }
    }
    BattleArena, Pit, TeamDeathMatch: {
        { SpawnPoint: FreeRespawnPoint },
        { RespawnTime: SetRespawnWaitingTime },
    }
    BattleArena, Pit, TeamDeathMatch, Elimination: {
        { GameLoseWin: GameLost, GameWon },
    }
    BattleArena, TeamDeathMatch: {
        { CurrentSurrenderVotes: SurrenderVoteStarted, CurrentSurrenderVotes },
        { SurrenderDeclined: SurrenderDeclined },
        { SurrenderTimes: SetSurrenderTimes },
    }
    BattleArena, TeamDeathMatch, Elimination: {
        { PlayerId: RemoteEnemySpotted }
    }
    BattleArena: {
        { GameLoseWin: GameLostBaseDestroyed, GameWonBaseDestroyed },
        { TeamBaseBoolean: PlayerInsideBase, TeamBaseContested },
        {
            TeamBaseState:
            TeamBaseState,
            TeamBaseCaptureStart,
            TeamBaseCaptureStop,
            TeamBaseSectionComplete,
            TeamBaseFinalSectionComplete,
            TeamBaseInitialise,
        }
    }

}

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GameTime(f32);

#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct HealedCubes {
    pub healed_machine: u16,
    pub type_performing_healing: TargetType,
    pub target_type: TargetType,
    #[byteserde(replace(hit_cubes.len()))]
    pub num_healed_cubes: u16,
    #[byteserde(deplete(num_healed_cubes as usize))]
    pub hit_cubes: Vec<HitCubeInfo>,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct CommandOnly;

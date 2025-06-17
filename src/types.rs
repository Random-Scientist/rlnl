use std::ops::{Deref, DerefMut};

use byteserde::{
    error::SerDesError,
    prelude::{ByteDeserializeSlice, ByteSerializeHeap},
};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeHeap};
use strum::FromRepr;

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct CompressedVec3<const FACTOR: u32> {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct CompressedQuat<const FACTOR: u32> {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct PosQuatPair {
    pos: CompressedVec3<768>,
    rot: CompressedQuat<4>,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct Byte3 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct SQuat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct ByteFloat<const FACTOR: u16>(u8);

impl<const FACTOR: u16> From<ByteFloat<FACTOR>> for f32 {
    fn from(value: ByteFloat<FACTOR>) -> Self {
        f32::from(value.0) * f32::from(FACTOR) / 255.0
    }
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct ShortFloat<const FACTOR: u16>(u8);

impl<const FACTOR: u16> From<ShortFloat<FACTOR>> for f32 {
    fn from(value: ShortFloat<FACTOR>) -> Self {
        f32::from(value.0) * f32::from(FACTOR) / 32767.0
    }
}

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct CapturePoint {
    pub pos: PosQuatPair,
    pub team: i8,
    pub progress: ByteFloat<4>,
    pub max_progress: ByteFloat<4>,
}

#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct HitCubeInfo {
    pub pos: Byte3,
    pub damage: i32,
}
#[derive(Debug, Default, Clone, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct IngamePlayerStats {
    pub player_name: u8,
    #[byteserde(replace(stats.len()))]
    num_stats: u8,
    #[byteserde(deplete(num_stats as usize))]
    pub stats: Vec<IngameStat>,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct IngameStat {
    pub id: IngameStatId,
    pub amount: u32,
    pub score: u32,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct CubeState {
    pub loc: Byte3,
    pub status: CubeStatus,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct ItemDescriptor {
    category: ItemCategory,
    size: ItemSize,
}
#[derive(Debug, Default, Clone, Copy, ByteDeserializeSlice, ByteSerializeHeap)]
pub struct GameModeSettings {
    game_time_minutes: i32,
    kill_limit: i32,
    respawn_heal_duration: f32,
    respawn_full_heal_duration: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CubeStatus {
    pub ty: CubeHistoryEventType,
    pub damage: Option<i32>,
}
impl ByteDeserializeSlice<CubeStatus> for CubeStatus {
    fn byte_deserialize(
        des: &mut byteserde::prelude::ByteDeserializerSlice,
    ) -> byteserde::error::Result<CubeStatus> {
        let ty = des.deserialize()?;
        let mut damage = None;
        if ty == CubeHistoryEventType::Heal {
            damage = Some(i32::from_le_bytes(*des.deserialize_bytes_array_ref::<4>()?));
        }
        Ok(Self { ty, damage })
    }
}
impl ByteSerializeHeap for CubeStatus {
    fn byte_serialize_heap(
        &self,
        ser: &mut byteserde::prelude::ByteSerializerHeap,
    ) -> byteserde::error::Result<()> {
        ser.serialize(&self.ty)?;
        if let Some(dmg) = self.damage {
            ser.serialize_bytes_slice(&dmg.to_le_bytes())?;
        }
        Ok(())
    }
}
#[derive(Debug, Default, Clone)]
pub struct BinaryWriterString(String);
const U8_HIGH_BIT: u8 = 0b10000000;

impl ByteDeserializeSlice<BinaryWriterString> for BinaryWriterString {
    fn byte_deserialize(
        des: &mut byteserde::prelude::ByteDeserializerSlice,
    ) -> byteserde::error::Result<BinaryWriterString> {
        let mut len = 0usize;
        let mut ct = 0;
        loop {
            let next = des.deserialize_u8()?;
            len |= ((next & 0x7F) as usize) << ct;
            ct += 7;
            if next & U8_HIGH_BIT == 0 {
                break;
            }
        }
        let slice = des.deserialize_bytes_slice(len * 2)?;
        let iter = (0..len).map(|i| u16::from_be_bytes([slice[2 * i], slice[2 * i + 1]]));
        Ok(Self(
            std::char::decode_utf16(iter)
                .collect::<Result<String, _>>()
                .map_err(|_| SerDesError {
                    message: "Failed to decode UTF16 C# string".into(),
                })?,
        ))
    }
}
impl ByteSerializeHeap for BinaryWriterString {
    fn byte_serialize_heap(
        &self,
        ser: &mut byteserde::prelude::ByteSerializerHeap,
    ) -> byteserde::error::Result<()> {
        let mut ctr = 0usize;
        let utf16 = self
            .0
            .encode_utf16()
            .flat_map(|v| {
                ctr += 1;
                v.to_le_bytes()
            })
            .collect::<Vec<_>>();

        while ctr >= U8_HIGH_BIT as usize {
            ser.serialize_bytes_slice(&[ctr as u8 & U8_HIGH_BIT])?;
            ctr >>= 7;
        }
        ser.serialize_bytes_slice(&[ctr as u8 & 0x7F])?;
        ser.serialize_bytes_slice(&utf16)?;
        Ok(())
    }
}
impl Deref for BinaryWriterString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BinaryWriterString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Debug, Default, Clone)]
pub struct StringCode {
    pub ty: GameServerErrorCodes,
    pub custom: Option<BinaryWriterString>,
}
impl ByteDeserializeSlice<StringCode> for StringCode {
    fn byte_deserialize(
        des: &mut byteserde::prelude::ByteDeserializerSlice,
    ) -> byteserde::error::Result<StringCode> {
        let ty = des.deserialize()?;

        Ok(Self {
            ty,
            custom: if ty == GameServerErrorCodes::StrErrCustomString {
                Some(des.deserialize()?)
            } else {
                None
            },
        })
    }
}
impl ByteSerializeHeap for StringCode {
    fn byte_serialize_heap(
        &self,
        ser: &mut byteserde::prelude::ByteSerializerHeap,
    ) -> byteserde::error::Result<()> {
        ser.serialize(&self.ty)?;
        if let Some(ref custom) = self.custom {
            ser.serialize(custom)?;
        }
        Ok(())
    }
}
macro_rules! enum_serialize {
    ($name:ty, $repr:ty) => {
        impl ByteDeserializeSlice<$name> for $name {
            fn byte_deserialize(
                des: &mut byteserde::prelude::ByteDeserializerSlice,
            ) -> byteserde::error::Result<$name> {
                <$name>::from_repr(<$repr>::from_le_bytes(
                    *des.deserialize_bytes_array_ref::<{ ::std::mem::size_of::<$repr>() }>()?,
                ))
                .ok_or(SerDesError {
                    message: ::std::stringify!("Invalid value for ", $name).into(),
                })
            }
        }
        impl ByteSerializeHeap for $name {
            fn byte_serialize_heap(
                &self,
                ser: &mut byteserde::prelude::ByteSerializerHeap,
            ) -> byteserde::error::Result<()> {
                let _ = ser.serialize_bytes_slice(&(*self as $repr).to_le_bytes())?;
                Ok(())
            }
        }
    };
}
#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PingType {
    MoveHere,
    GoingHere,
    Danger,
    #[default]
    Unknown,
}
enum_serialize! { PingType, i32 }

#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum ItemCategory {
    #[default]
    NotAFunctionalItem = 0,
    Wheel = 1,
    Hover = 2,
    Wing = 3,
    Rudder = 4,
    Thruster = 5,
    InsectLeg = 6,
    MechLeg = 7,
    Ski = 8,
    TankTrack = 9,
    Rotor = 10,
    SprinterLeg = 11,
    Propeller = 12,
    Laser = 100,
    Plasma = 200,
    Mortar = 250,
    Rail = 300,
    Nano = 400,
    Tesla = 500,
    Aeroflak = 600,
    Ion = 650,
    Seeker = 701,
    Chaingun = 750,
    ShieldModule = 800,
    GhostModule = 801,
    BlinkModule = 802,
    EmpModule = 803,
    WindowmakerModule = 804,
    EnergyModule = 900,
}

enum_serialize! { ItemCategory, i16}
#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GameEndReason {
    #[default]
    TimeOut,
    NoPlayersRemaining,
    OneTeamRemaining,
    BaseCaptured,
    BaseDestroyed,
    PitMaxKillsAchieved,
    TeamDeathMatchMaxKillsAchieved,
    TeamDeathMatchTimeExpiredSuddenDeath,
    TeamDeathMatchTimeExpiredMostKills,
    Surrendered,
}
enum_serialize! { GameEndReason, u8 }

#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum ItemSize {
    #[default]
    NotAWeapon = 0,
    T0 = 100,
    T1 = 200,
    T2 = 300,
    T3 = 400,
    T4 = 500,
    T5 = 600,
}
enum_serialize! { ItemSize, i16}
#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VoteType {
    #[default]
    BestPlayed,
    BestLooking,
}
enum_serialize! { VoteType, u32 }
#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CubeHistoryEventType {
    Destroy = 1,
    #[default]
    Heal = 2,
}
enum_serialize! { CubeHistoryEventType, u8 }

#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GameServerErrorCodes {
    StrErrHaxSpeed = 0,
    StrErrHaxException = 1,
    StrErrHaxTeleport = 2,
    StrErrHaxEacViolation = 6,
    StrErrHaxAfk = 7,
    StrErrHaxFirerange = 8,
    StrErrHaxFiredamage = 9,
    StrErrHaxFirerate = 10,
    StrErrHaxFireposition = 11,
    StrErrIncorrectGameGuid = 12,
    StrErrCustomString = 13,
    StrErrTimedOut = 14,
    #[default]
    StrErrGameEnded = 15,
}
enum_serialize! { GameServerErrorCodes, i32 }

#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TargetType {
    Player = 0,
    #[default]
    Environment = 1,
    TeamBase = 2,
    FusionShield = 3,
    EqualizerCrystal = 4,
    CapturePoint = 5,
}
enum_serialize! { TargetType, u8 }
#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EqualizerState {
    // starts
    Start = 1,
    // end types
    #[default]
    Lost = 2,
    Defended = 3,
    Destroyed = 4,
}
enum_serialize! { EqualizerState, u8 }

#[derive(Debug, Default, Clone, Copy, FromRepr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IngameStatId {
    #[default]
    None,
    DestroyedCubes,
    DestroyedCubesInProtection,
    DestroyedCubesDefendingTheBase,
    Kill,
    KillAssist,
    HealCubes,
    HealAssist,
    DestroyedProtoniumCubes,
    BaseCaptureClassicMode,
    RobotDestroyed,
    Score,
    HealthPercentageBonusClassicMode,
    Points,
    CurrentKillStreak,
    BestKillStreak,
    CapturePointBattleArenaMode,
    EqualiserDestroyedBattleArenaMode,
    BattleArenaObjectives,
}
enum_serialize! { IngameStatId, u8 }

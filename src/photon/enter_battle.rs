use std::collections::HashMap;

use polariton::operation::{Arr, Dict, ParameterTable, Typed};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GameModeType {
    BattleArena,
    SuddenDeath,
    Pit,
    TestMode,
    SinglePlayerTDM,
    TeamDeathmatch,
    Campaign,
}
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub network_channel_type: String,
    pub max_sent_message_queue_size: u16,
    pub is_acks_long: bool,
    pub network_drop_threshold: u8,
    pub packet_size: u16,
    pub max_combined_reliable_message_count: u16,
    pub max_combined_reliable_message_size: u16,
    pub min_update_timeout: u16,
    pub max_delay: i64,
    pub overflow_threshold: u8,
    pub max_packet_size: u16,
    pub resend_delay_base: f64,
    pub resend_delay_rtt_mult: f64,
    pub network_peer_update_interval: i64,
    pub max_milliseconds_delay_for_being_disconnected: i32,
}
fn str<C>(val: impl AsRef<str>) -> Typed<C> {
    Typed::Str(val.as_ref().to_string().into())
}
impl NetworkConfig {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(
            vec![
                (str("NetworkChannelType"), str(&self.network_channel_type)),
                (
                    str("MaxSentMessageQueueSize"),
                    Typed::Long(self.max_sent_message_queue_size.into()),
                ),
                (str("IsAcksLong"), Typed::Long(self.is_acks_long.into())),
                (
                    str("NetworkDropThreshold"),
                    Typed::Long(self.network_drop_threshold.into()),
                ),
                (str("PacketSize"), Typed::Long(self.packet_size.into())),
                (
                    str("MaxCombinedReliableMessageCount"),
                    Typed::Long(self.max_combined_reliable_message_count.into()),
                ),
                (
                    str("MaxCombinedReliableMessageSize"),
                    Typed::Long(self.max_combined_reliable_message_size.into()),
                ),
                (
                    str("MinUpdateTimeout"),
                    Typed::Long(self.min_update_timeout.into()),
                ),
                (str("MaxDelay"), Typed::Long(self.max_delay)),
                (
                    str("OverflowThreshold"),
                    Typed::Long(self.overflow_threshold.into()),
                ),
                (
                    str("MaxPacketSize"),
                    Typed::Long(self.max_packet_size.into()),
                ),
                (
                    str("ResendDelayBase"),
                    Typed::Double(self.resend_delay_base),
                ),
                (
                    str("ResendDelayRttMult"),
                    Typed::Double(self.resend_delay_rtt_mult),
                ),
                (
                    str("NetworkPeerUpdateInterval"),
                    Typed::Double(self.network_peer_update_interval as f64),
                ),
                (
                    str("MaxMillisecondsDelayForBeingDisconnected"),
                    Typed::Long(self.max_milliseconds_delay_for_being_disconnected.into()),
                ),
            ]
            .into(),
        )
    }
}
#[derive(Debug, Clone)]
pub struct PlayerData {
    pub name: String,
    pub display_name: String,
    pub robot_name: String,
    pub cube_map: Vec<u8>,
    pub color_map: Vec<u8>,
    pub spawn_effect: Option<String>,
    pub death_effect: Option<String>,
    pub group_id: Option<String>,
    pub team: i32,
    pub has_premium: bool,
    pub weapon_order: Vec<i32>,
    pub robot_unique_id: String,
    pub cpu: i32,
    //pub use_custom_avatar: bool,
    pub non_custom_avatar_id: Option<i32>,
    pub mastery_level: i32,
    pub tier: i32,
    pub player_rank: i32,
    pub weapon_ranks: Vec<(i32, i32)>,
    pub is_ai: bool,
    pub clan_info: Option<ClanInfo>,
}

impl PlayerData {
    fn as_transmissible<C>(&self) -> Typed<C> {
        let mut map = vec![
            (str("name"), str(&self.name)),
            (str("displayName"), str(&self.display_name)),
            (str("robotName"), str(&self.robot_name)),
            (str("cubeMap"), Typed::Bytes(self.cube_map.clone().into())),
            (str("colorMap"), Typed::Bytes(self.color_map.clone().into())),
            (str("team"), Typed::Int(self.team)),
            (str("hasPremium"), Typed::Bool(self.has_premium)),
            (
                str("weaponOrder"),
                Typed::IntArr(self.weapon_order.clone().into()),
            ),
            (str("robotUniqueID"), str(&self.robot_unique_id)),
            (str("cpu"), Typed::Int(self.cpu)),
            (
                str("useCustomAvatar"),
                Typed::Bool(self.non_custom_avatar_id.is_none()),
            ),
            (str("masteryLevel"), Typed::Int(self.mastery_level)),
            (str("tier"), Typed::Int(self.tier)),
            (str("playerRank"), Typed::Int(self.player_rank)),
            (
                str("weaponRanks"),
                Typed::Dict(Dict {
                    key_ty: polariton::serdes::TypePrefix::Int,
                    val_ty: polariton::serdes::TypePrefix::Int,
                    items: self
                        .weapon_ranks
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (Typed::Int(k), Typed::Int(v)))
                        .collect(),
                }),
            ),
            (str("isAI"), Typed::Bool(self.is_ai)),
        ];
        self.spawn_effect
            .as_ref()
            .inspect(|&e| map.push((str("spawnEffect"), str(e))));
        self.death_effect
            .as_ref()
            .inspect(|&e| map.push((str("deathEffect"), str(e))));
        self.group_id
            .as_ref()
            .inspect(|&e| map.push((str("groupId"), str(e))));
        self.non_custom_avatar_id
            .inspect(|&v| map.push((str("avatarId"), Typed::Int(v))));
        self.clan_info.as_ref().inspect(|&v| {
            map.push((str("clanName"), str(&v.name)));
            map.push((str("clanUseCustomAvatar"), Typed::Bool(v.use_custom_avatar)));
            map.push((str("clanDefaultAvatarID"), Typed::Int(v.default_avatar_id)));
        });
        Typed::HashMap(map.into())
    }
}
#[derive(Debug, Clone)]
pub struct ClanInfo {
    pub name: String,
    pub use_custom_avatar: bool,
    pub default_avatar_id: i32,
}
#[derive(Debug, Clone)]
pub struct BattleParametersData {
    pub host_ip: String,
    pub host_port: i32,
    pub map_name: String,
    pub game_mode: GameModeKey,
    pub network_config: NetworkConfig,
    pub game_guid: String,
    pub map_visibility: Option<i32>,
    pub is_autoheal: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct GameModeKey {
    pub ty: GameModeType,
    pub is_ranked: bool,
    pub is_custom: bool,
}
impl BattleParametersData {
    fn insert_params<C>(&self, dict: &mut ParameterTable<C>) {
        const GAME_MODE_KEY: u8 = 1;
        const HOST_IP_KEY: u8 = 6;
        const HOST_PORT_KEY: u8 = 7;
        const MAP_NAME_KEY: u8 = 8;
        const NETWORK_CONFIG_KEY: u8 = 23;
        const IS_RANKED_KEY: u8 = 25;
        const IS_CUSTOM_GAME_KEY: u8 = 27;
        const MAP_VISIBILITY_KEY: u8 = 28;
        const GAME_GUID_KEY: u8 = 40;
        dict.insert(HOST_IP_KEY, str(&self.host_ip));
        dict.insert(HOST_PORT_KEY, Typed::Int(self.host_port));
        dict.insert(MAP_NAME_KEY, str(&self.map_name));
        dict.insert(GAME_MODE_KEY, Typed::Int((self.game_mode.ty as u8).into()));
        dict.insert(GAME_GUID_KEY, str(&self.game_guid));
        dict.insert(IS_RANKED_KEY, Typed::Bool(self.game_mode.is_ranked));
        dict.insert(IS_CUSTOM_GAME_KEY, Typed::Bool(self.game_mode.is_custom));
        dict.insert(NETWORK_CONFIG_KEY, self.network_config.as_transmissible());
        if let Some(vis) = self.map_visibility {
            dict.insert(MAP_VISIBILITY_KEY, Typed::Int(vis));
        }
    }
}
#[derive(Debug, Clone)]
pub struct EnterBattle {
    pub params: BattleParametersData,
    pub players: Vec<PlayerData>,
}
impl EnterBattle {
    pub fn as_param_table<C>(&self) -> ParameterTable<C> {
        const PLAYERS_KEY: u8 = 5;
        let mut table = ParameterTable::from_dict(HashMap::new());
        self.params.insert_params(&mut table);
        table.insert(
            PLAYERS_KEY,
            Typed::Arr(Arr {
                ty: polariton::serdes::TypePrefix::HashMap,
                items: self
                    .players
                    .iter()
                    .map(PlayerData::as_transmissible)
                    .collect(),
            }),
        );
        table
    }
}

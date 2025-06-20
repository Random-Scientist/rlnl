#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkEvent {
    //NOT_IMPLEMENTED_ReleaseLocalAI = 62,
    //NOT_USED_SelfExitedCapturePoint = 96,
    //NO_LONGER_USED_SpottedByStructure = 139,
    OnFailedToConnectToMasterServer = 0,
    OnConnectingToLobbyServer = 1,
    OnConnectedToLobbyServer = 2,
    OnDisconnectingFromLobbyServer = 3,
    OnDisconnectedFromLobbyServer = 4,
    OnConnectedToServer = 5,
    OnFailedToConnectToServer = 6,
    OnConnectionLost = 7,
    OnDisconnectedFromServer = 8,
    OnConnectedToGameServer = 9,
    OnServerStarted = 10,
    OnServerStopped = 11,
    OnPlayerConnectedToServer = 12,
    OnPlayerDisconnectedFromServer = 13,
    RequestRespawnPoint = 14,
    FreeSpawnPoint = 15,
    RequestTeamBaseModel = 16,
    RequestCapturePoints = 17,
    RequestEqualizerModel = 18,
    TeamBase = 19,
    RegisterCapturePoints = 20,
    RegisterEqualizer = 21,
    FreeRespawnPoint = 22,
    PlayerIDs = 23,
    SyncMachineCubes = 24,
    MachineDestroyed = 25,
    GameStarted = 26,
    OnPlayerInputChanged = 27,
    OnServerReceivedInputChange = 28,
    ClientUnregistered = 29,
    OnAnotherClientDisconnected = 30,
    OnClientReconnected = 31,
    DamageCube = 32,
    FireWeaponEffect = 33,
    FireMiss = 35,
    MultipleFireMisses = 36,
    CurrentGameTime = 37,
    EndGame = 38,
    GameWon = 39,
    GameLost = 40,
    GameWonBaseDestroyed = 41,
    GameLostBaseDestroyed = 42,
    BuffTeamPlayers = 43,
    PlayerThreateningBase = 44,
    TimeToGameStart = 45,
    SetRespawnWaitingTime = 46,
    TeamBaseState = 47,
    TeamBaseCaptureStart = 48,
    TeamBaseCaptureReset = 49,
    TeamBaseCaptureStop = 50,
    TeamBaseSectionComplete = 51,
    TeamBaseFinalSectionComplete = 52,
    TeamBaseInitialise = 53,
    GetClientPings = 54,
    SetClientPing = 55,
    WarnPlayer = 56,
    EnemySpotted = 57,
    RemoteEnemySpotted = 58,
    AssistBonusRequest = 59,
    TeamBaseContested = 60,
    AcquireRemoteAI = 61,
    KillBonusRequest = 63,
    EACRegisterToken = 64,
    HeallingAssistBonusRequest = 65,
    ProtectTeamMateBonusRequest = 66,
    DefendTheBaseBonusRequest = 67,
    DestroyCubesBonusRequest = 68,
    DestroyHealCubesPointsAwarded = 69,
    ConfirmedKill = 70,
    BonusesFlushDone = 71,
    AlignmentRectifierStarted = 72,
    HealCubesBonusRequest = 73,
    SetShieldState = 74,
    TeamBaseLowHealth = 75,
    AwardTeamBaseProtoniumDestroyedRequest = 76,
    InitialiseGameStats = 77,
    UpdateGameStats = 78,
    MapPingEvent = 79,
    SurrenderRequest = 80,
    HealSelf = 81,
    HealSelfResponse = 82,
    SurrenderVoteStarted = 83,
    SurrenderVoteCast = 84,
    CurrentSurrenderVotes = 85,
    SurrenderAccepted = 86,
    SurrenderDeclined = 87,
    SetSurrenderTimes = 88,
    SetFinalGameScore = 89,
    PitLeaderBoardUpdate = 90,
    PitModeState = 91,
    ValidateGameGuid = 93,
    GameGuidValidated = 94,
    PlayerInsideBase = 95,
    ConfirmedAssist = 97,
    LockOnNotification = 98,
    LockOnNotificationBroadcast = 99,
    ShieldSpawned = 100,
    SpawnShield = 101,
    BroadcastOpenShield = 102,
    OpenShield = 103,
    BroadcastInvisible = 104,
    MakeInvisible = 105,
    BroadcastVisible = 106,
    MakeVisible = 107,
    BroadcastActivateTeleportEffect = 108,
    ActivateTeleportEffect = 109,
    BroadcastActivateReadyEffect = 110,
    ActivateReadyEffect = 111,
    BroadcastSpawnEmpLocator = 112,
    SpawnEmpLocator = 113,
    BroadcastSpawnEmpMachineEffect = 114,
    SpawnEmpMachineEffect = 115,
    WeaponSelect = 116,
    BroadcastWeaponSelect = 117,
    HostAIs = 118,
    HealAlly = 119,
    HealAllyResponse = 120,
    SelfDestructClassicMode = 121,
    GameModeSettings = 122,
    TeamDeathMatchState = 123,
    ClientDisconnecting = 124,
    SendBonus = 125,
    TestConnection = 126,
    MachineDestroyedConfirmed = 127,
    EnergyModuleActivated = 128,
    BroadcastLoadingProgress = 129,
    RequestLoadingProgressAllUsers = 130,
    LoadingComplete = 131,
    GameAborted = 132,
    SendDamagedByEnemyShield = 134,
    DamagedByEnemyShield = 135,
    EqualizerNotification = 136,
    CapturePointProgress = 137,
    CapturePointNotification = 138,
    RadarModuleActivated = 140,
    RemoteRadarModuleActivated = 141,
    EACMessage = 142,
    Taunt = 143,
    MachineFullHealth = 144,
    RequestSync = 145,
    BeginSync = 146,
    EndOfSync = 147,
    SyncTeamBaseCubes = 148,
    SyncEqualizerNotification = 150,
    PlayerQuitRequest = 151,
    PlayerQuitRequestComplete = 152,
    DamageCubeEffectOnly = 153,
    DamageCubeNoEffect = 154,
    DestroyCubeEffectOnly = 155,
    DestroyCubeNoEffect = 156,
    DestroyCubesFull = 157,
    LongPlayValue = 170,
    UpdateVotingAfterBattle = 171,
    CosmeticAction = 172,
}

use serde::{Deserialize, Serialize};

pub mod achievements;
pub mod auth;
pub mod broadcast;
pub mod chat;
pub mod chunk;
pub mod commerce;
pub mod common;
pub mod entitlements;
pub mod errors;
pub mod friends;
pub mod game;
pub mod groups;
pub mod invites;
pub mod overlay;
pub mod permissions;
pub mod presence;
pub mod profile;
pub mod steam;
pub mod system;
pub mod user;
pub mod voip;

use crate::protocol::achievements::*;
use crate::protocol::auth::*;
use crate::protocol::broadcast::*;
use crate::protocol::chat::*;
use crate::protocol::chunk::*;
use crate::protocol::commerce::*;
use crate::protocol::common::*;
use crate::protocol::entitlements::*;
use crate::protocol::friends::*;
use crate::protocol::game::*;
use crate::protocol::groups::*;
use crate::protocol::invites::*;
use crate::protocol::overlay::*;
use crate::protocol::permissions::*;
use crate::protocol::presence::*;
use crate::protocol::profile::*;
use crate::protocol::steam::*;
use crate::protocol::system::*;
use crate::protocol::user::*;
use crate::protocol::voip::*;
#[cfg(feature = "client")]
use crate::request_response;

// Model definitions

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "LSX")]
pub struct Lsx {
    #[serde(rename = "$value")]
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Message {
    Request(Request),
    Response(Response),
    Event(Event),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@recipient", default)]
    pub recipient: String,
    #[serde(rename = "$value")]
    pub body: RequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@sender", default)]
    pub sender: String,
    #[serde(rename = "$value")]
    pub body: ResponseBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "@sender", default)]
    pub sender: String,
    #[serde(rename = "$value")]
    pub body: EventBody,
}

// Message definitions

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestBody {
    AcceptFriendInvite(AcceptFriendInvite),
    AcceptInvite(AcceptInvite),
    AddRecentPlayers(AddRecentPlayers),
    AreChunksInstalled(AreChunksInstalled),
    BlockUser(BlockUser),
    BroadcastStart(BroadcastStart),
    BroadcastStop(BroadcastStop),
    ChallengeResponse(ChallengeResponse),
    CheckPermission(CheckPermission),
    Checkout(Checkout),
    ConsumeEntitlement(ConsumeEntitlement),
    CreateChunk(CreateChunk),
    CreateGroup(CreateGroup),
    DetermineCommerceCurrency(DetermineCommerceCurrency),
    EnableVoip(EnableVoip),
    EnterGroup(EnterGroup),
    ExtendTrial(ExtendTrial),
    GetAllGameInfo(GetAllGameInfo),
    GetAuthCode(GetAuthCode),
    GetAuthToken(GetAuthToken),
    GetBlockList(GetBlockList),
    GetBroadcastStatus(GetBroadcastStatus),
    GetCatalog(GetCatalog),
    GetChunkPriority(GetChunkPriority),
    GetConfig(GetConfig),
    GetGameInfo(GetGameInfo),
    GetGroupInfo(GetGroupInfo),
    GetInternetConnectedState(GetInternetConnectedState),
    GetPresence(GetPresence),
    GetPresenceVisibility(GetPresenceVisibility),
    GetProfile(GetProfile),
    GetSetting(GetSetting),
    GetSettings(GetSettings),
    GetStore(GetStore),
    GetUserProfileByEmailorEaid(GetUserProfileByEmailorEaid),
    GetUtcTime(GetUtcTime),
    GetVoipStatus(GetVoipStatus),
    GetWalletBalance(GetWalletBalance),
    GoOnline(GoOnline),
    GrantAchievement(GrantAchievement),
    InvalidateLicense(InvalidateLicense),
    InviteUsersToGroup(InviteUsersToGroup),
    IsFileDownloaded(IsFileDownloaded),
    IsProgressiveInstallationAvailable(IsProgressiveInstallationAvailable),
    LeaveGroup(LeaveGroup),
    Logout(Logout),
    MuteUser(MuteUser),
    OverlayStateChanged(OverlayStateChanged),
    PostAchievementEvents(PostAchievementEvents),
    PostWincodes(PostWincodes),
    QueryAchievements(QueryAchievements),
    QueryAreFriends(QueryAreFriends),
    QueryCategories(QueryCategories),
    QueryChunkFiles(QueryChunkFiles),
    QueryChunkStatus(QueryChunkStatus),
    QueryContent(QueryContent),
    QueryEntitlements(QueryEntitlements),
    QueryFriends(QueryFriends),
    QueryGroup(QueryGroup),
    QueryImage(QueryImage),
    QueryManifest(QueryManifest),
    QueryMuteState(QueryMuteState),
    QueryOffers(QueryOffers),
    QueryPresence(QueryPresence),
    RefreshEntitlements(RefreshEntitlements),
    RemoveFriend(RemoveFriend),
    RemoveUsersFromGroup(RemoveUsersFromGroup),
    RequestFriend(RequestFriend),
    RequestLicense(RequestLicense),
    RestartGame(RestartGame),
    SelectStore(SelectStore),
    SendChatMessage(SendChatMessage),
    SendGameMessage(SendGameMessage),
    SendGroupGameInvite(SendGroupGameInvite),
    SendInvite(SendInvite),
    SetChunkPriority(SetChunkPriority),
    SetDlcInstalledState(SetDlcInstalledState),
    SetDownloaderUtilization(SetDownloaderUtilization),
    SetPresence(SetPresence),
    SetPresenceVisibility(SetPresenceVisibility),
    SetSteamLocale(SetSteamLocale),
    ShowIgo(ShowIgo),
    ShowIgoWindow(ShowIgoWindow),
    StartDownload(StartDownload),
    StartGame(StartGame),
    SteamAchievementErrorTelemetry(SteamAchievementErrorTelemetry),
    SteamPurchaseConfirmation(SteamPurchaseConfirmation),
    SubscribePresence(SubscribePresence),
    UnblockUser(UnblockUser),
    UnsubscribePresence(UnsubscribePresence),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseBody {
    Achievement(Achievement),
    AchievementSets(AchievementSets),
    AreChunksInstalledResponse(AreChunksInstalledResponse),
    AuthCode(AuthCode),
    AuthToken(AuthToken),
    BroadcastStatus(BroadcastStatus),
    ChallengeAccepted(ChallengeAccepted),
    CheckPermissionResponse(CheckPermissionResponse),
    ConsumeEntitlementResponse(ConsumeEntitlementResponse),
    CreateChunkResponse(CreateChunkResponse),
    ErrorSuccess(ErrorSuccess),
    ExtendTrialResponse(ExtendTrialResponse),
    GetAllGameInfoResponse(GetAllGameInfoResponse),
    GetBlockListResponse(GetBlockListResponse),
    GetCatalogResponse(GetCatalogResponse),
    GetChunkPriorityResponse(GetChunkPriorityResponse),
    GetConfigResponse(GetConfigResponse),
    GetGameInfoResponse(GetGameInfoResponse),
    GetPresenceResponse(GetPresenceResponse),
    GetPresenceVisibilityResponse(GetPresenceVisibilityResponse),
    GetProfileResponse(GetProfileResponse),
    GetSettingResponse(GetSettingResponse),
    GetSettingsResponse(GetSettingsResponse),
    GetStoreResponse(GetStoreResponse),
    GetUserProfileByEmailorEaidResponse(GetUserProfileByEmailorEaidResponse),
    GetUtcTimeResponse(GetUtcTimeResponse),
    GetVoipStatusResponse(GetVoipStatusResponse),
    GetWalletBalanceResponse(GetWalletBalanceResponse),
    GroupEnterEvent(GroupEnterEvent),
    GroupInfo(GroupInfo),
    InternetConnectedState(InternetConnectedState),
    IsFileDownloadedResponse(IsFileDownloadedResponse),
    IsProgressiveInstallationAvailableResponse(IsProgressiveInstallationAvailableResponse),
    QueryAreFriendsResponse(QueryAreFriendsResponse),
    QueryCategoriesResponse(QueryCategoriesResponse),
    QueryChunkFilesResponse(QueryChunkFilesResponse),
    QueryChunkStatusResponse(QueryChunkStatusResponse),
    QueryContentResponse(QueryContentResponse),
    QueryEntitlementsResponse(QueryEntitlementsResponse),
    QueryFriendsResponse(QueryFriendsResponse),
    QueryGroupResponse(QueryGroupResponse),
    QueryImageResponse(QueryImageResponse),
    QueryManifestResponse(QueryManifestResponse),
    QueryMuteStateResponse(QueryMuteStateResponse),
    QueryOffersResponse(QueryOffersResponse),
    QueryPresenceResponse(QueryPresenceResponse),
    RequestLicenseResponse(RequestLicenseResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventBody {
    AchievementSets(AchievementSets),
    BlockListUpdated(BlockListUpdated),
    BroadcastEvent(BroadcastEvent),
    Challenge(Challenge),
    ChatMessageEvent(ChatMessageEvent),
    ChatStateUpdateEvent(ChatStateUpdateEvent),
    ChunkStatus(ChunkStatus),
    CoreContentUpdated(CoreContentUpdated),
    CurrentUserPresenceEvent(CurrentUserPresenceEvent),
    FriendsEvent(FriendsEvent),
    GameMessageEvent(GameMessageEvent),
    GetPresenceResponse(GetPresenceResponse),
    GroupEnterEvent(GroupEnterEvent),
    GroupEvent(GroupEvent),
    GroupInviteEvent(GroupInviteEvent),
    GroupLeaveEvent(GroupLeaveEvent),
    IgoEvent(IgoEvent),
    IgoUnavailable(IgoUnavailable),
    Login(Login),
    MinimizeRequest(MinimizeRequest),
    MultiplayerInvite(MultiplayerInvite),
    MultiplayerInvitePending(MultiplayerInvitePending),
    OnlineStatusEvent(OnlineStatusEvent),
    PresenceEvent(PresenceEvent),
    PresenceVisibilityEvent(PresenceVisibilityEvent),
    ProfileEvent(ProfileEvent),
    PurchaseEvent(PurchaseEvent),
    QueryEntitlementsResponse(QueryEntitlementsResponse),
    QueryFriendsResponse(QueryFriendsResponse),
    RestoreRequest(RestoreRequest),
    SteamAchievementEvent(SteamAchievementEvent),
    SteamActivateOverlayToStoreEvent(SteamActivateOverlayToStoreEvent),
    UserInvitedEvent(UserInvitedEvent),
    VoipStatusEvent(VoipStatusEvent),
}

// Request -> Response mapping

#[cfg(feature = "client")]
pub trait RequestResponse {
    type Response;
    fn extract_response(body: ResponseBody) -> Result<Self::Response, crate::sdk::SdkError>;
}

#[cfg(feature = "client")]
request_response! {
    AcceptFriendInvite => ErrorSuccess,
    AcceptInvite => ErrorSuccess,
    AddRecentPlayers => ErrorSuccess,
    AreChunksInstalled => AreChunksInstalledResponse,
    BlockUser => ErrorSuccess,
    BroadcastStart => ErrorSuccess,
    BroadcastStop => ErrorSuccess,
    ChallengeResponse => ChallengeAccepted,
    CheckPermission => CheckPermissionResponse,
    Checkout => ErrorSuccess,
    ConsumeEntitlement => ConsumeEntitlementResponse,
    CreateChunk => CreateChunkResponse,
    CreateGroup => GroupEnterEvent,
    DetermineCommerceCurrency => ErrorSuccess,
    EnableVoip => ErrorSuccess,
    EnterGroup => GroupInfo,
    ExtendTrial => ExtendTrialResponse,
    GetAllGameInfo => GetAllGameInfoResponse,
    GetAuthCode => AuthCode,
    GetAuthToken => AuthToken,
    GetBlockList => GetBlockListResponse,
    GetBroadcastStatus => BroadcastStatus,
    GetCatalog => GetCatalogResponse,
    GetChunkPriority => GetChunkPriorityResponse,
    GetConfig => GetConfigResponse,
    GetGameInfo => GetGameInfoResponse,
    GetGroupInfo => GroupEnterEvent,
    GetInternetConnectedState => InternetConnectedState,
    GetPresence => GetPresenceResponse,
    GetPresenceVisibility => GetPresenceVisibilityResponse,
    GetProfile => GetProfileResponse,
    GetSetting => GetSettingResponse,
    GetSettings => GetSettingsResponse,
    GetStore => GetStoreResponse,
    GetUserProfileByEmailorEaid => GetUserProfileByEmailorEaidResponse,
    GetUtcTime => GetUtcTimeResponse,
    GetVoipStatus => GetVoipStatusResponse,
    GetWalletBalance => GetWalletBalanceResponse,
    GoOnline => ErrorSuccess,
    GrantAchievement => Achievement,
    InvalidateLicense => ErrorSuccess,
    InviteUsersToGroup => ErrorSuccess,
    IsFileDownloaded => IsFileDownloadedResponse,
    IsProgressiveInstallationAvailable => IsProgressiveInstallationAvailableResponse,
    LeaveGroup => ErrorSuccess,
    Logout => ErrorSuccess,
    MuteUser => ErrorSuccess,
    OverlayStateChanged => ErrorSuccess,
    PostAchievementEvents => ErrorSuccess,
    PostWincodes => ErrorSuccess,
    QueryAchievements => AchievementSets,
    QueryAreFriends => QueryAreFriendsResponse,
    QueryCategories => QueryCategoriesResponse,
    QueryChunkFiles => QueryChunkFilesResponse,
    QueryChunkStatus => QueryChunkStatusResponse,
    QueryContent => QueryContentResponse,
    QueryEntitlements => QueryEntitlementsResponse,
    QueryFriends => QueryFriendsResponse,
    QueryGroup => QueryGroupResponse,
    QueryImage => QueryImageResponse,
    QueryManifest => QueryManifestResponse,
    QueryMuteState => QueryMuteStateResponse,
    QueryOffers => QueryOffersResponse,
    QueryPresence => QueryPresenceResponse,
    RefreshEntitlements => ErrorSuccess,
    RemoveFriend => ErrorSuccess,
    RemoveUsersFromGroup => ErrorSuccess,
    RequestFriend => ErrorSuccess,
    RequestLicense => RequestLicenseResponse,
    RestartGame => ErrorSuccess,
    SelectStore => ErrorSuccess,
    SendChatMessage => ErrorSuccess,
    SendGameMessage => ErrorSuccess,
    SendGroupGameInvite => ErrorSuccess,
    SendInvite => ErrorSuccess,
    SetChunkPriority => ErrorSuccess,
    SetDlcInstalledState => ErrorSuccess,
    SetDownloaderUtilization => ErrorSuccess,
    SetPresence => ErrorSuccess,
    SetPresenceVisibility => ErrorSuccess,
    SetSteamLocale => ErrorSuccess,
    ShowIgo => ErrorSuccess,
    ShowIgoWindow => ErrorSuccess,
    StartDownload => ErrorSuccess,
    StartGame => ErrorSuccess,
    SteamAchievementErrorTelemetry => ErrorSuccess,
    SteamPurchaseConfirmation => ErrorSuccess,
    SubscribePresence => ErrorSuccess,
    UnblockUser => ErrorSuccess,
    UnsubscribePresence => ErrorSuccess,
}

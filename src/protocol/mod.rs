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
pub mod presence;
pub mod profile;
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
use crate::protocol::presence::*;
use crate::protocol::profile::*;
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
    AreChunksInstalled(AreChunksInstalled),
    BlockUser(BlockUser),
    BroadcastStart(BroadcastStart),
    BroadcastStop(BroadcastStop),
    ChallengeResponse(ChallengeResponse),
    Checkout(Checkout),
    ConsumeEntitlement(ConsumeEntitlement),
    CreateChunk(CreateChunk),
    CreateGroup(CreateGroup),
    EnableVoip(EnableVoip),
    EnterGroup(EnterGroup),
    ExtendTrial(ExtendTrial),
    GetAuthCode(GetAuthCode),
    GetAllGameInfo(GetAllGameInfo),
    GetBlockList(GetBlockList),
    GetChunkPriority(GetChunkPriority),
    GetConfig(GetConfig),
    GetGroupInfo(GetGroupInfo),
    GetInternetConnectedState(GetInternetConnectedState),
    GetPresence(GetPresence),
    GetPresenceVisibility(GetPresenceVisibility),
    GetProfile(GetProfile),
    GetSettings(GetSettings),
    GetUserProfileByEmailorEaid(GetUserProfileByEmailorEaid),
    GetUtcTime(GetUtcTime),
    GetVoipStatus(GetVoipStatus),
    GetWalletBalance(GetWalletBalance),
    GoOnline(GoOnline),
    GrantAchievement(GrantAchievement),
    InviteUsersToGroup(InviteUsersToGroup),
    IsFileDownloaded(IsFileDownloaded),
    IsProgressiveInstallationAvailable(IsProgressiveInstallationAvailable),
    LeaveGroup(LeaveGroup),
    MuteUser(MuteUser),
    PostAchievementEvents(PostAchievementEvents),
    QueryAchievements(QueryAchievements),
    QueryAreFriends(QueryAreFriends),
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
    RemoveFriend(RemoveFriend),
    RemoveUsersFromGroup(RemoveUsersFromGroup),
    RequestFriend(RequestFriend),
    RestartGame(RestartGame),
    SendChatMessage(SendChatMessage),
    SendGameMessage(SendGameMessage),
    SendGroupGameInvite(SendGroupGameInvite),
    SendInvite(SendInvite),
    SetChunkPriority(SetChunkPriority),
    SetDownloaderUtilization(SetDownloaderUtilization),
    SetPresence(SetPresence),
    SetPresenceVisibility(SetPresenceVisibility),
    ShowIgo(ShowIgo),
    ShowIgoWindow(ShowIgoWindow),
    StartGame(StartGame),
    UnblockUser(UnblockUser),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseBody {
    Achievement(Achievement),
    AchievementSets(AchievementSets),
    AreChunksInstalledResponse(AreChunksInstalledResponse),
    AuthCode(AuthCode),
    ChallengeAccepted(ChallengeAccepted),
    ConsumeEntitlementResponse(ConsumeEntitlementResponse),
    CreateChunkResponse(CreateChunkResponse),
    ErrorSuccess(ErrorSuccess),
    ExtendTrialResponse(ExtendTrialResponse),
    GetAllGameInfoResponse(GetAllGameInfoResponse),
    GetBlockListResponse(GetBlockListResponse),
    GetChunkPriorityResponse(GetChunkPriorityResponse),
    GetConfigResponse(GetConfigResponse),
    GetPresenceResponse(GetPresenceResponse),
    GetPresenceVisibilityResponse(GetPresenceVisibilityResponse),
    GetProfileResponse(GetProfileResponse),
    GetSettingsResponse(GetSettingsResponse),
    GetUtcTimeResponse(GetUtcTimeResponse),
    GetUserProfileByEmailorEaidResponse(GetUserProfileByEmailorEaidResponse),
    GetVoipStatusResponse(GetVoipStatusResponse),
    GetWalletBalanceResponse(GetWalletBalanceResponse),
    GroupInfo(GroupInfo),
    InternetConnectedState(InternetConnectedState),
    IsFileDownloadedResponse(IsFileDownloadedResponse),
    IsProgressiveInstallationAvailableResponse(IsProgressiveInstallationAvailableResponse),
    QueryAreFriendsResponse(QueryAreFriendsResponse),
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
    PresenceVisibilityEvent(PresenceVisibilityEvent),
    ProfileEvent(ProfileEvent),
    PurchaseEvent(PurchaseEvent),
    QueryEntitlementsResponse(QueryEntitlementsResponse),
    QueryFriendsResponse(QueryFriendsResponse),
    RestoreRequest(RestoreRequest),
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
    AreChunksInstalled => AreChunksInstalledResponse,
    BlockUser => ErrorSuccess,
    BroadcastStart => ErrorSuccess,
    BroadcastStop => ErrorSuccess,
    ChallengeResponse => ChallengeAccepted,
    Checkout => ErrorSuccess,
    ConsumeEntitlement => ConsumeEntitlementResponse,
    CreateChunk => CreateChunkResponse,
    CreateGroup => GroupInfo,
    EnableVoip => ErrorSuccess,
    EnterGroup => GroupInfo,
    ExtendTrial => ExtendTrialResponse,
    GetAuthCode => AuthCode,
    GetAllGameInfo => GetAllGameInfoResponse,
    GetBlockList => GetBlockListResponse,
    GetChunkPriority => GetChunkPriorityResponse,
    GetConfig => GetConfigResponse,
    GetGroupInfo => GroupInfo,
    GetInternetConnectedState => InternetConnectedState,
    GetPresence => GetPresenceResponse,
    GetPresenceVisibility => GetPresenceVisibilityResponse,
    GetProfile => GetProfileResponse,
    GetSettings => GetSettingsResponse,
    GetUserProfileByEmailorEaid => GetUserProfileByEmailorEaidResponse,
    GetUtcTime => GetUtcTimeResponse,
    GetVoipStatus => GetVoipStatusResponse,
    GetWalletBalance => GetWalletBalanceResponse,
    GoOnline => ErrorSuccess,
    GrantAchievement => Achievement,
    InviteUsersToGroup => ErrorSuccess,
    IsFileDownloaded => IsFileDownloadedResponse,
    IsProgressiveInstallationAvailable => IsProgressiveInstallationAvailableResponse,
    LeaveGroup => ErrorSuccess,
    MuteUser => ErrorSuccess,
    PostAchievementEvents => ErrorSuccess,
    QueryAchievements => AchievementSets,
    QueryAreFriends => QueryAreFriendsResponse,
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
    RemoveFriend => ErrorSuccess,
    RemoveUsersFromGroup => ErrorSuccess,
    RequestFriend => ErrorSuccess,
    RestartGame => ErrorSuccess,
    SendChatMessage => ErrorSuccess,
    SendGameMessage => ErrorSuccess,
    SendGroupGameInvite => ErrorSuccess,
    SendInvite => ErrorSuccess,
    SetChunkPriority => ErrorSuccess,
    SetDownloaderUtilization => ErrorSuccess,
    SetPresence => ErrorSuccess,
    SetPresenceVisibility => ErrorSuccess,
    ShowIgo => ErrorSuccess,
    ShowIgoWindow => ErrorSuccess,
    StartGame => ErrorSuccess,
    UnblockUser => ErrorSuccess,
}

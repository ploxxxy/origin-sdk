use serde::{Deserialize, Serialize};

use super::model::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestBody {
    GetInternetConnectedState(GetInternetConnectedState),
    QueryFriends(QueryFriends),
    ChallengeResponse(ChallengeResponse),
    GetConfig(GetConfig),
    GetProfile(GetProfile),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseBody {
    ChallengeAccepted(ChallengeAccepted),
    InternetConnectedState(InternetConnectedState),
    GetConfigResponse(GetConfigResponse),
    GetProfileResponse(GetProfileResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventBody {
    Challenge(Challenge),
    PresenceVisibilityEvent(PresenceVisibilityEvent),
    CurrentUserPresenceEvent(CurrentUserPresenceEvent),
}

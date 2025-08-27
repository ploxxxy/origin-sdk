use serde::{Deserialize, Serialize};

use super::model::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestBody {
    GetInternetConnectedState(GetInternetConnectedState),
    QueryFriends(QueryFriends),
    ChallengeResponse(ChallengeResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseBody {
    ChallengeAccepted(ChallengeAccepted),
    InternetConnectedState(InternetConnectedState),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventBody {
    Challenge(Challenge),
    PresenceVisibilityEvent(PresenceVisibilityEvent),
}

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

const ERR_BASE: i32 = -0x60000000;
const WRN_BASE: i32 = 0x40000000;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum ErrorArea {
    General = 0,
    Sdk = 1 << 16,
    Core = 2 << 16,
    Igo = 3 << 16,
    Friends = 4 << 16,
    Presence = 5 << 16,
    Commerce = 6 << 16,
    Achievements = 7 << 16,
    Lsx = 8 << 16,
    Proxy = 9 << 16,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum ErrorLevel {
    L0 = 0 << 24,
    L1 = 1 << 24,
    L2 = 2 << 24,
    L3 = 3 << 24,
    L4 = 4 << 24,
}

const fn make_err(level: ErrorLevel, area: ErrorArea, code: i32) -> i32 {
    ERR_BASE | (level as i32) | (area as i32) | code
}

const fn make_wrn(level: ErrorLevel, area: ErrorArea, code: i32) -> i32 {
    WRN_BASE | (level as i32) | (area as i32) | code
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum OriginError {
    Success = 0,
    Pending = 1,
    General = -1,

    InvalidHandle = make_err(ErrorLevel::L1, ErrorArea::General, 0),
    OutOfMemory = make_err(ErrorLevel::L0, ErrorArea::General, 1),
    NotImplemented = make_err(ErrorLevel::L1, ErrorArea::General, 2),
    InvalidUser = make_err(ErrorLevel::L2, ErrorArea::General, 3),
    InvalidArgument = make_err(ErrorLevel::L2, ErrorArea::General, 4),
    NoCallbackSpecified = make_err(ErrorLevel::L2, ErrorArea::General, 5),
    BufferTooSmall = make_err(ErrorLevel::L2, ErrorArea::General, 6),
    TooManyValuesInList = make_err(ErrorLevel::L2, ErrorArea::General, 7),
    NotFound = make_err(ErrorLevel::L2, ErrorArea::General, 8),
    InvalidPersona = make_err(ErrorLevel::L2, ErrorArea::General, 9),
    NoNetwork = make_err(ErrorLevel::L2, ErrorArea::General, 10),
    NoService = make_err(ErrorLevel::L2, ErrorArea::General, 11),
    NotLoggedIn = make_err(ErrorLevel::L2, ErrorArea::General, 12),
    MandatoryOriginUpdatePending = make_err(ErrorLevel::L2, ErrorArea::General, 13),
    AccountInUse = make_err(ErrorLevel::L2, ErrorArea::General, 14),
    TooManyInstances = make_err(ErrorLevel::L1, ErrorArea::General, 15),
    AlreadyExists = make_err(ErrorLevel::L3, ErrorArea::General, 16),
    InvalidOperation = make_err(ErrorLevel::L2, ErrorArea::General, 17),
    AgeRestricted = make_err(ErrorLevel::L2, ErrorArea::General, 18),
    Banned = make_err(ErrorLevel::L2, ErrorArea::General, 19),
    NotReady = make_err(ErrorLevel::L2, ErrorArea::General, 20),

    SdkNotInitialized = make_err(ErrorLevel::L0, ErrorArea::Sdk, 0),
    SdkInvalidAllocatorDeallocatorCombination = make_err(ErrorLevel::L1, ErrorArea::Sdk, 1),
    SdkIsRunning = make_err(ErrorLevel::L2, ErrorArea::Sdk, 2),
    SdkNotAllResourcesReleased = make_err(ErrorLevel::L3, ErrorArea::Sdk, 3),
    SdkInvalidResource = make_err(ErrorLevel::L2, ErrorArea::Sdk, 4),
    SdkInternalError = make_err(ErrorLevel::L1, ErrorArea::Sdk, 5),
    SdkInternalBufferTooSmall = make_err(ErrorLevel::L1, ErrorArea::Sdk, 6),

    WarningSdkAlreadyInitialized = make_wrn(ErrorLevel::L2, ErrorArea::Sdk, 1),
    WarningSdkStillRunning = make_wrn(ErrorLevel::L2, ErrorArea::Sdk, 2),
    WarningSdkEnumeratorInUse = make_wrn(ErrorLevel::L2, ErrorArea::Sdk, 3),
    WarningSdkEnumeratorTerminated = make_wrn(ErrorLevel::L2, ErrorArea::Sdk, 4),

    CoreNotloaded = make_err(ErrorLevel::L0, ErrorArea::Core, 0),
    CoreLoginFailed = make_err(ErrorLevel::L1, ErrorArea::Core, 1),
    CoreAuthenticationFailed = make_err(ErrorLevel::L1, ErrorArea::Core, 2),
    CoreSendFailed = make_err(ErrorLevel::L1, ErrorArea::Core, 4),
    CoreReceiveFailed = make_err(ErrorLevel::L1, ErrorArea::Core, 5),
    CoreResourceNotFound = make_err(ErrorLevel::L1, ErrorArea::Core, 6),
    CoreIncompatibleVersion = make_err(ErrorLevel::L0, ErrorArea::Core, 7),
    CoreNotInstalled = make_err(ErrorLevel::L0, ErrorArea::Core, 8),

    WarningIgoNotloaded = make_wrn(ErrorLevel::L1, ErrorArea::Igo, 0),
    WarningIgoSupportNotloaded = make_wrn(ErrorLevel::L1, ErrorArea::Igo, 1),
    IgoIllegalAnchorPoint = make_err(ErrorLevel::L3, ErrorArea::Igo, 2),
    IgoIllegalDockPoint = make_err(ErrorLevel::L3, ErrorArea::Igo, 3),
    IgoNotAvailable = make_err(ErrorLevel::L3, ErrorArea::Igo, 4),

    NoMultiplayerId = make_err(ErrorLevel::L2, ErrorArea::Presence, 0),

    LsxInvalidResponse = make_err(ErrorLevel::L2, ErrorArea::Lsx, 0),
    LsxNoResponse = make_err(ErrorLevel::L1, ErrorArea::Lsx, 1),
    LsxInvalidRequest = make_err(ErrorLevel::L2, ErrorArea::Lsx, 2),

    CommerceNoSuchStore = make_err(ErrorLevel::L1, ErrorArea::Commerce, 0),
    CommerceNoSuchCatalog = make_err(ErrorLevel::L1, ErrorArea::Commerce, 1),
    CommerceInvalidReply = make_err(ErrorLevel::L1, ErrorArea::Commerce, 2),
    CommerceNoCategories = make_err(ErrorLevel::L2, ErrorArea::Commerce, 3),
    CommerceNoProducts = make_err(ErrorLevel::L2, ErrorArea::Commerce, 4),
    CommerceUnderageUser = make_err(ErrorLevel::L2, ErrorArea::Commerce, 5),
    CommerceDeprecatedStore = make_err(ErrorLevel::L2, ErrorArea::Commerce, 6),

    Proxy = make_err(ErrorLevel::L2, ErrorArea::Proxy, 0),
    SuccessProxyOk = make_wrn(ErrorLevel::L4, ErrorArea::Proxy, 200),
    SuccessProxyCreated = make_wrn(ErrorLevel::L4, ErrorArea::Proxy, 201),
    SuccessProxyAccepted = make_wrn(ErrorLevel::L4, ErrorArea::Proxy, 202),
    SuccessProxyNonAuthInfo = make_wrn(ErrorLevel::L4, ErrorArea::Proxy, 203),
    SuccessProxyNoContent = make_wrn(ErrorLevel::L4, ErrorArea::Proxy, 204),
    SuccessResetContent = make_wrn(ErrorLevel::L4, ErrorArea::Proxy, 205),
    SuccessPartialContent = make_wrn(ErrorLevel::L4, ErrorArea::Proxy, 206),
    ProxyBadRequest = make_err(ErrorLevel::L2, ErrorArea::Proxy, 400),
    ProxyUnauthorized = make_err(ErrorLevel::L2, ErrorArea::Proxy, 401),
    ProxyPaymentRequired = make_err(ErrorLevel::L2, ErrorArea::Proxy, 402),
    ProxyForbidden = make_err(ErrorLevel::L2, ErrorArea::Proxy, 403),
    ProxyNotFound = make_err(ErrorLevel::L2, ErrorArea::Proxy, 404),
    ProxyMethodNotAllowed = make_err(ErrorLevel::L2, ErrorArea::Proxy, 405),
    ProxyNotAcceptable = make_err(ErrorLevel::L2, ErrorArea::Proxy, 406),
    ProxyRequestTimeout = make_err(ErrorLevel::L2, ErrorArea::Proxy, 408),
    ProxyConflict = make_err(ErrorLevel::L2, ErrorArea::Proxy, 409),
    ProxyInternalError = make_err(ErrorLevel::L2, ErrorArea::Proxy, 500),
    ProxyNotImplemented = make_err(ErrorLevel::L2, ErrorArea::Proxy, 501),
    ProxyBadGateway = make_err(ErrorLevel::L2, ErrorArea::Proxy, 502),
    ProxyServiceUnavailable = make_err(ErrorLevel::L2, ErrorArea::Proxy, 503),
    ProxyGatewayTimeout = make_err(ErrorLevel::L2, ErrorArea::Proxy, 504),
}

impl OriginError {
    /// Returns true if this is a success code (0) or pending (1)
    pub const fn is_success(&self) -> bool {
        matches!(self, OriginError::Success | OriginError::Pending)
    }

    /// Returns true if this is a warning
    pub const fn is_warning(&self) -> bool {
        (*self as i32) > 1
    }

    /// Returns true if this is an error
    pub const fn is_error(&self) -> bool {
        (*self as i32) < 0
    }

    /// Extract the error area from the error code
    pub fn error_area(&self) -> Option<ErrorArea> {
        ErrorArea::from_i32((*self as i32) & 0x00FF0000)
    }

    /// Extract the error level from the error code
    pub fn error_level(&self) -> Option<ErrorLevel> {
        ErrorLevel::from_i32((*self as i32) & 0x0F000000)
    }

    /// Extract the specific error code within the area
    pub const fn error_code(&self) -> i32 {
        (*self as i32) & 0x0000FFFF
    }
}

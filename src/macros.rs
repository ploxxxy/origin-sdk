#[macro_export]
macro_rules! request_response {
    ($($request:ident => $response:ident),* $(,)?) => {
        $(
            impl RequestResponse for $request {
                type Response = $response;

                fn extract_response(body: ResponseBody) -> Result<Self::Response, $crate::sdk::SdkError> {

                    // Handle ErrorSuccess response first
                    if let ResponseBody::ErrorSuccess(error) = &body {
                        if error.code.is_success() {
                            // Received success, but it's only valid if we're expecting an ErrorSuccess
                            // Fall through to let the second match handle it

                            if error.code == $crate::protocol::errors::OriginError::Pending {
                                tracing::info!("Request is pending: {}", error.description);
                            }
                        } else if error.code.is_error() {
                            return Err($crate::sdk::SdkError::OriginError(error.code, error.description.clone()));
                        } else if error.code.is_warning() {
                            tracing::warn!("Request returned warning: {:?} - {}", error.code, error.description);
                        }
                    }

                    match body {
                        ResponseBody::$response(response) => Ok(response),
                        _ => Err($crate::sdk::SdkError::Other("Wrong response type".to_string())),
                    }
                }
            }

            impl From<$request> for RequestBody {
                fn from(val: $request) -> Self {
                    RequestBody::$request(val)
                }
            }
        )*
    };
}

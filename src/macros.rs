#[macro_export]
macro_rules! request_response {
    ($($request:ident => $response:ident),* $(,)?) => {
        $(
            impl RequestResponse for $request {
                type Response = $response;

                fn extract_response(body: ResponseBody) -> Result<Self::Response, $crate::sdk::SdkError> {

                    // Handle error response first
                    if let ResponseBody::ErrorSuccess(error) = &body {
                        match error.code {
                            $crate::protocol::errors::OriginError::OriginSuccess => {
                                // Success code, but only valid if we're expecting an ErrorSuccess
                                // Fall through to let the match handle it
                            }
                            $crate::protocol::errors::OriginError::OriginPending => {
                                tracing::warn!("Request is pending: {}", error.description);
                            }
                            _ => {
                                return Err($crate::sdk::SdkError::OriginError(error.code, error.description.clone()));
                            }
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

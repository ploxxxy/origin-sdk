#[macro_export]
macro_rules! request_response {
    ($($request:ident => $response:ident),* $(,)?) => {
        $(
            impl RequestResponse for $request {
                type Response = $response;

                fn extract_response(body: ResponseBody) -> Result<Self::Response, $crate::sdk::SdkError> {
                    match body {
                        ResponseBody::$response(response) => Ok(response),
                        _ => Err("Wrong response type".into()),
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

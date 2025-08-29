#[macro_export]
macro_rules! xml {
    ($name:ident { $($field:tt)* }) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name { $($field)* }
    };
}

#[macro_export]
macro_rules! request_response {
    ($($request:ident => $response:ident),* $(,)?) => {
        $(
            impl RequestResponse for $request {
                type Response = $response;
            }

            impl From<$request> for RequestBody {
                fn from(val: $request) -> Self {
                    RequestBody::$request(val)
                }
            }

            impl TryFrom<ResponseBody> for $response {
                type Error = $crate::sdk::SdkError;

                fn try_from(body: ResponseBody) -> Result<Self, Self::Error> {
                    match body {
                        ResponseBody::$response(response) => Ok(response),
                        // TODO: construct MismatchedResponseBody or similar
                        _ => Err("Wrong response type".into()),
                    }
                }
            }
        )*
    };
}

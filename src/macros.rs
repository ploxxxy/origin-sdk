#[macro_export]
macro_rules! xml {
    ($name:ident { $($field:tt)* }) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name { $($field)* }
    };
}

#[macro_export]
macro_rules! match_response {
    ($response:expr, $expected_variant:ident) => {
        match $response {
            ResponseBody::$expected_variant(inner) => Ok(inner),
            _ => Err(format!(
                "Unexpected response. Was expecting {}",
                stringify!($expected_variant)
            )
            .into()),
        }
    };
}

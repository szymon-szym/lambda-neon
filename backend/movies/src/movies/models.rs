use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Movie {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) short_description: String,
}


#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub (crate) struct TitanResponse {
    pub(crate) embedding: Vec<f32>,
    input_text_token_count: i128,
}

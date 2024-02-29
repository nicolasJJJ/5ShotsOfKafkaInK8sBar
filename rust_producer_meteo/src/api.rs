use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigAPI {
    pub apis: Vec<ApiParameters>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiParameters {
    pub name: String,
    pub url: String,
    pub params: Vec<HashMap<String, String>>,
}

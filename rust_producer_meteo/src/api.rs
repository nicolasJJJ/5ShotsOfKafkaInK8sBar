use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigAPI {
    pub apis: Vec<ApiParameters>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiParameters {
    pub name: String,
    pub api : String,
    pub url: String,
    pub params: Vec<HashMap<String, String>>,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Apikey {
    pub keys: Vec<HashMap<String, String>>,
}

impl Apikey {
    pub fn find_value_by_key(&self, search_key: &str) -> Option<String> {
        for key_map in &self.keys {
            if let Some(value) = key_map.get(search_key) {
                return Some(value.clone());
            }
        }
        None
    }

}
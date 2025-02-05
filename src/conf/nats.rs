#![allow(dead_code)]

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Nats {
    pub url: String,
}

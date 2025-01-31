use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

pub const REGISTER_CODE_EXPIRE_TIME: u64 = 60 * 5;

#[derive(Debug, EnumString, EnumVariantNames, Display, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Bilibili,
    Youtube,
}

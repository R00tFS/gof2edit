use crate::binary_version::BinaryVersion;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Deserialize, Serialize)]
pub struct BinaryPatch {
    pub name: String,
    pub author: String,
    pub description: String,
    pub addresses: HashMap<BinaryVersion, BTreeMap<u64, u8>>,
}

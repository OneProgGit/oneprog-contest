#![cfg(feature = "server")]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: Uuid,
    pub exp: usize,
}

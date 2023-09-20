use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Genre {
    pub genre_id:   i32,
    pub genre_name: String,
}
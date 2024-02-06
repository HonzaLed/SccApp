use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConfigTOML {
    pub login: Login,
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
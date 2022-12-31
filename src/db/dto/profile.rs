use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub id: String,
    pub name: String,
}

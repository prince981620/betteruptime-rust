use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct CraeteWebsiteInput {
    pub url: String
}
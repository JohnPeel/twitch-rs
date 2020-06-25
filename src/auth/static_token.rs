use std::error::Error;
use async_trait::async_trait;

use crate::{AccessToken, AuthProvider};

pub struct StaticAuthProvider {
    client_id: String,
    access_token: String
}

impl StaticAuthProvider {
    pub fn new(client_id: String, access_token: String) -> Self {
        Self {
            client_id,
            access_token
        }
    }
}

#[async_trait(?Send)]
impl AuthProvider for StaticAuthProvider {
    fn get_client_id(&self) -> Option<String> {
        Some(self.client_id.clone())
    }

    fn can_refresh(&self) -> bool {
        false
    }

    async fn get_token(&self, scopes: Option<Vec<String>>) -> Result<AccessToken, Box<dyn Error>> {
        Ok(AccessToken {
            access_token: self.access_token.clone(),
            refresh_token: None,
            expires_in: None,
            scopes,
            time: None
        })
    }

    async fn refresh_token(&self, _: &mut AccessToken, _: Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}

use std::error::Error;
use tokio::time::Instant;
use async_trait::async_trait;
use oauth2::{
    AsyncClientCredentialsTokenRequest,
    AsyncRefreshTokenRequest,
    AuthUrl,
    ClientId,
    ClientSecret,
    TokenResponse,
    Scope,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::{AuthType, reqwest::async_http_client, RefreshToken};
use crate::{auth::{AccessToken, AuthProvider}, api::ApiEndpoint};


pub struct ClientCredentialsAuthProvider {
    client_id: String,
    client: BasicClient
}

impl ClientCredentialsAuthProvider {
    pub fn new(client_id: String, client_secret: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client_id: client_id.clone(),
            client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(ApiEndpoint::Auth.get_endpoint("authorize")?.into_string())?,
                Some(TokenUrl::new(ApiEndpoint::Auth.get_endpoint("token")?.into_string())?)
            ).set_auth_type(AuthType::RequestBody)
        })
    }
}

#[async_trait(?Send)]
impl AuthProvider for ClientCredentialsAuthProvider {
    fn get_client_id(&self) -> Option<String> {
        Some(self.client_id.clone())
    }

    fn can_refresh(&self) -> bool {
        true
    }

    async fn get_token(&self, scopes: Option<Vec<String>>) -> Result<AccessToken, Box<dyn Error>> {
        let mut request = self.client
            .exchange_client_credentials();

        if let Some(scopes) = scopes {
            for scope in scopes.into_iter() {
                request = request.add_scope(Scope::new(scope));
            }
        }

        let result = request
            .request_async(async_http_client)
            .await
            .map_err(|e| format!("{}", e))?;

        Ok(AccessToken {
            access_token: result.access_token().secret().to_owned(),
            refresh_token: result.refresh_token().to_owned().map(|x| x.secret().to_owned()),
            expires_in: result.expires_in().to_owned(),
            scopes: result.scopes().to_owned()
                .map(|x| x.to_owned().into_iter().map(|x| x.into()).collect()),
            time: Some(Instant::now())
        })
    }

    async fn refresh_token(&self, access_token: &mut AccessToken, scopes: Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
        let refresh_token = RefreshToken::new(access_token.refresh_token.clone().unwrap());
        let mut request = self.client
            .exchange_refresh_token(&refresh_token);

        if let Some(scopes) = scopes {
            for scope in scopes.into_iter() {
                request = request.add_scope(Scope::new(scope));
            }
        }

        let result = request
            .request_async(async_http_client)
            .await
            .map_err(|e| format!("{}", e))?;
        
        access_token.access_token = result.access_token().secret().to_owned();
        access_token.refresh_token = result.refresh_token().to_owned().map(|x| x.secret().to_owned());
        access_token.expires_in = result.expires_in().to_owned();
        access_token.scopes = result.scopes().to_owned()
            .map(|x| x.to_owned().into_iter().map(|x| x.into()).collect());
        access_token.time = Some(Instant::now());
        Ok(())
    }
}
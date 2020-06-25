
use std::{cell::RefCell, error::Error, rc::Rc};
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};

pub use crate::auth::*;
use crate::api::ApiEndpoint;

#[cfg(feature = "clips")]
use crate::api::helix::clips::ClipsGroup;
#[cfg(feature = "search")]
use crate::api::helix::search::SearchGroup;

#[derive(Default)]
pub struct TwitchClientBuilder {
    client: Option<Client>,
    client_id: Option<String>,
    access_token: Option<String>,
    client_secret: Option<String>,
    scopes: Option<Vec<String>>
}

impl TwitchClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn with_client_id<S: Into<String>>(mut self, client_id: S) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn with_access_token<S: Into<String>>(mut self, access_token: S) -> Self {
        self.access_token = Some(access_token.into());
        self
    }

    pub fn with_client_secret<S: Into<String>>(mut self, client_secret: S) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = Some(scopes);
        self
    }

    pub async fn build(self) -> Result<TwitchClient, Box<dyn Error>> {
        let mut auth_provider: Option<Box<dyn AuthProvider>> = None;
        let mut access_token = AccessToken::default();

        if let Some(client_id) = self.client_id {
            if let Some(access_token) = self.access_token {
                auth_provider = Some(Box::new(StaticAuthProvider::new(client_id, access_token)));
            } else if let Some(client_secret) = self.client_secret {
                auth_provider = Some(Box::new(ClientCredentialsAuthProvider::new(client_id, client_secret)?));
            }

            if let Some(ref auth_provider) = auth_provider {
                access_token = auth_provider.get_token(self.scopes).await?;
            }
        }

        if auth_provider.is_none() {
            return Err(String::from("Unable to create auth provider!").into());
        }

        let inner = Rc::new(TwitchClientInner {
            client: self.client.unwrap_or_default(),
            auth_provider,
            access_token: RefCell::new(access_token)
        });

        Ok(TwitchClient {
            _client: Rc::clone(&inner),
            #[cfg(feature = "clips")]
            clips: ClipsGroup::new(Rc::clone(&inner)),
            #[cfg(feature = "search")]
            search: SearchGroup::new(Rc::clone(&inner))
        })
    }
}

pub(crate) struct TwitchClientInner {
    client: Client,
    auth_provider: Option<Box<dyn AuthProvider>>,
    access_token: RefCell<AccessToken>
}

pub struct TwitchClient {
    _client: Rc<TwitchClientInner>,

    #[cfg(feature = "clips")]
    pub clips: ClipsGroup,

    #[cfg(feature = "search")]
    pub search: SearchGroup
}

impl TwitchClientInner {
    pub(crate) async fn call<T: DeserializeOwned, S: Into<String>, Q: Serialize + ?Sized>(&self, endpoint: ApiEndpoint, method: Method, scopes: Option<Vec<String>>, url: S, query: &Q) -> Result<T, Box<dyn Error>> {
        let mut request = self.client
            .request(method, endpoint.get_endpoint(&url.into())?)
            .header("Accept", "application/json")
            .query(query);

        //println!("{:?}", &request);
        
        if let Some(auth_provider) = &self.auth_provider {
            if let Some(ref client_id) = auth_provider.get_client_id() {
                request = request.header("Client-Id", client_id);
            }

            if auth_provider.can_refresh() {
                let mut needs_refresh = false;
                {
                    let access_token = &*self.access_token.borrow();

                    if access_token.refresh_token.is_some() {
                        if let Some(ref time) = access_token.time {
                            if let Some(ref expires_in) = access_token.expires_in {
                                needs_refresh = &time.elapsed() >= expires_in;
                            }
                        }
                    }
                }

                if needs_refresh {
                    auth_provider.refresh_token(&mut *self.access_token.borrow_mut(), scopes).await?;
                }
            }

            let access_token = &*self.access_token.borrow();

            // FIXME: Check scope!

            request = request.header("Authorization", format!("Bearer {}", access_token.access_token));
        }

        let response = request.send().await?;

        //println!("{:?}", &response);

        Ok(response.json()
            .await?)
    }
}

impl TwitchClient {
    pub fn builder() -> TwitchClientBuilder {
        TwitchClientBuilder::new()
    }
}

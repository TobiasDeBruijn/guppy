use color_eyre::eyre::Error;
use color_eyre::Result;
use jwt_simple::prelude::*;
use reqwest::Client;

pub struct GToken {
    token: String,
    expires_at: i64,
}

pub struct ServiceAccount {
    pub private_key_pem: String,
    pub email: String,
    pub scopes: String,
}

#[derive(Serialize, Deserialize)]
struct CustomGoogleClaims {
    scope: String,
}

const JWT_AUDIENCE: &str = "https://oauth2.googleapis.com/token";
const OAUTH_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";
const GOOGLE_OAUTH_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

#[derive(Serialize)]
struct OauthTokenForm<'a> {
    grant_type: &'a str,
    assertion: &'a str,
}

#[derive(Deserialize)]
struct OauthTokenResponse {
    access_token: String,
    expires_in: i64,
}

impl GToken {
    pub async fn create(service_account: &ServiceAccount) -> Result<Self> {
        let custom_claims = CustomGoogleClaims {
            scope: service_account.scopes.clone(),
        };

        let claims = Claims::with_custom_claims(custom_claims, Duration::from_hours(1))
            .with_issuer(&service_account.email)
            .with_audience(JWT_AUDIENCE);

        let key_pair = RS256KeyPair::from_pem(&service_account.private_key_pem)
            .map_err(|e| Error::msg(format!("Failed to create key pair: {e}")))?;

        let jwt = key_pair
            .sign(claims)
            .map_err(|e| Error::msg(format!("Failed to sign JWT claims: {e}")))?;

        let response: OauthTokenResponse = Client::new()
            .post(GOOGLE_OAUTH_TOKEN_URL)
            .form(&OauthTokenForm {
                grant_type: OAUTH_GRANT_TYPE,
                assertion: &jwt,
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(Self {
            token: response.access_token,
            expires_at: time::OffsetDateTime::now_utc().unix_timestamp() + response.expires_in,
        })
    }

    pub fn token(&self) -> Option<&str> {
        if !self.is_valid() {
            return None;
        }

        Some(&self.token)
    }

    pub fn is_valid(&self) -> bool {
        time::OffsetDateTime::now_utc().unix_timestamp() < self.expires_at
    }
}

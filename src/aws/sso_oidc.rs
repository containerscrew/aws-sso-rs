use crate::aws::DeviceAuthCredentials;
use crate::aws::dto::DeviceClientCredentials;
use aws_config::SdkConfig;
use aws_config::retry::{RetryConfig, RetryMode};
use aws_sdk_ssooidc as sso_oidc;
use std::error::Error;
use std::time::Duration;
use tokio::time::{Instant, sleep};
use tracing::debug;

/// Seconds added to the polling interval each time AWS answers `SlowDownException`
/// (RFC 8628, section 3.5).
const SLOW_DOWN_STEP_SECS: u64 = 5;

pub async fn ssoidc_client(config: &SdkConfig, retries: u32) -> sso_oidc::Client {
    sso_oidc::Client::from_conf(
        sso_oidc::config::Builder::from(config)
            .retry_config(
                RetryConfig::standard()
                    .with_max_attempts(retries)
                    .with_retry_mode(RetryMode::Standard),
            )
            .build(),
    )
}

pub async fn register_device_credentials(
    client: &sso_oidc::client::Client,
) -> Result<DeviceClientCredentials, sso_oidc::Error> {
    let client_registration = client
        .register_client()
        .client_name("aws-sso-rs")
        .client_type("public")
        .send()
        .await?;

    // Parse client id and secret
    let client_id = client_registration.client_id.unwrap();
    let client_secret = client_registration.client_secret.unwrap();

    Ok(DeviceClientCredentials {
        client_id,
        client_secret,
    })
}

pub async fn get_device_authorization_credentials(
    client: &sso_oidc::client::Client,
    device_credentials: &DeviceClientCredentials,
    start_url: &String,
) -> Result<DeviceAuthCredentials, sso_oidc::Error> {
    let get_device_authorization = client
        .start_device_authorization()
        .client_id(&device_credentials.client_id)
        .client_secret(&device_credentials.client_secret)
        .start_url(start_url)
        .send()
        .await?;

    Ok(DeviceAuthCredentials {
        user_code: get_device_authorization.user_code.unwrap(),
        device_code: get_device_authorization.device_code.unwrap(),
        verification_url: get_device_authorization.verification_uri_complete.unwrap(),
        interval: get_device_authorization.interval as u64,
        expires_in: get_device_authorization.expires_in as u64,
    })
}

/// Makes a single `create_token` request for the device code.
///
/// It does not wait for the user: if the approval is still pending in the browser it returns
/// the `AuthorizationPendingException` error. Use [`poll_token`] to wait for the approval.
pub async fn generate_token(
    client: &sso_oidc::client::Client,
    device_client_credentials: &DeviceClientCredentials,
    device_auth_credentials: &DeviceAuthCredentials,
) -> Result<String, sso_oidc::Error> {
    // Generate and return the token
    let generate_token_output = client
        .create_token()
        .client_id(&device_client_credentials.client_id)
        .client_secret(&device_client_credentials.client_secret)
        .set_grant_type(Some(
            "urn:ietf:params:oauth:grant-type:device_code".to_owned(),
        ))
        .device_code(&device_auth_credentials.device_code)
        .send()
        .await?;

    Ok(generate_token_output.access_token.unwrap())
}

/// Polls [`generate_token`] until the user approves the device authorization in the browser.
///
/// Follows the device flow of RFC 8628: it waits `interval` seconds (as returned by AWS)
/// before each request, adds [`SLOW_DOWN_STEP_SECS`] seconds to the interval on
/// `SlowDownException`, and keeps going on `AuthorizationPendingException`. It gives up when
/// the `expires_in` deadline passes, when AWS reports the authorization as expired, when the
/// user denies it, or on any other error.
pub async fn poll_token(
    client: &sso_oidc::client::Client,
    device_client_credentials: &DeviceClientCredentials,
    device_auth_credentials: &DeviceAuthCredentials,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // AWS should never return 0, but a zero interval would spin the loop
    let mut interval = device_auth_credentials.interval.max(1);
    let deadline = Instant::now() + Duration::from_secs(device_auth_credentials.expires_in);

    loop {
        if Instant::now() >= deadline {
            return Err("Device authorization expired. Run aws-sso-rs again".into());
        }

        // Sleep first: the user is still approving the request in the browser
        sleep(Duration::from_secs(interval)).await;

        match generate_token(client, device_client_credentials, device_auth_credentials).await {
            Ok(token) => return Ok(token),
            Err(sso_oidc::Error::AuthorizationPendingException(_)) => {
                debug!("Authorization pending, next attempt in {interval}s");
            }
            Err(sso_oidc::Error::SlowDownException(_)) => {
                interval += SLOW_DOWN_STEP_SECS;
                debug!("AWS asked to slow down, next attempt in {interval}s");
            }
            Err(sso_oidc::Error::ExpiredTokenException(_)) => {
                return Err("Device authorization expired. Run aws-sso-rs again".into());
            }
            Err(sso_oidc::Error::AccessDeniedException(_)) => {
                return Err("Authorization denied in the browser".into());
            }
            Err(err) => return Err(err.into()),
        }
    }
}

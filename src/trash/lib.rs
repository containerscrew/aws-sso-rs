// use aws_config::meta::region::RegionProviderChain;
// use aws_config::retry::RetryConfig;
// use aws_config::SdkConfig;
// use aws_sdk_sso as sso;
// use aws_sdk_sso::config::Region;
// use aws_sdk_sso::types::AccountInfo;
// use aws_sdk_ssooidc as ssooidc;
// use aws_sdk_ssooidc::{config, Client};
// 
// pub struct DeviceClientCredentials {
//     client_id: String,
//     client_secret: String,
// }
// 
// pub struct DeviceAuthCredentials {
//     user_code: String,
//     device_code: String,
//     pub verification_url: String,
// }
// 
// #[derive(Debug)]
// pub struct AccountCredentials {
//     pub account_name: String,
//     pub role_name: String,
//     pub aws_access_key_id: String,
//     pub aws_secret_access_key: String,
//     pub aws_session_token: String,
// }
//
//
// pub async fn ssoidc_client(config: &SdkConfig, retries: u32) -> Client {
//     Client::from_conf(
//         config::Builder::from(config)
//             .retry_config(RetryConfig::standard().with_max_attempts(retries))
//             .build(),
//     )
// }
//
// pub async fn sso_client(config: &SdkConfig, retries: u32) -> sso::Client {
//     sso::Client::from_conf(
//         sso::config::Builder::from(config)
//             .retry_config(RetryConfig::standard().with_max_attempts(retries))
//             .build(),
//     )
// }
// 
// pub async fn get_register_device_credentials(
//     client: &ssooidc::client::Client,
// ) -> Result<DeviceClientCredentials, ssooidc::Error> {
//     let client_registration = client
//         .register_client()
//         .client_name("aws-sso-auth")
//         .client_type("public")
//         .send()
//         .await?;
// 
//     // Parse client id and secret
//     let client_id = client_registration.client_id.unwrap();
//     let client_secret = client_registration.client_secret.unwrap();
// 
//     Ok(DeviceClientCredentials {
//         client_id,
//         client_secret,
//     })
// }
// 
// pub async fn get_device_authorization_credentials(
//     client: &ssooidc::client::Client,
//     device_credentials: &DeviceClientCredentials,
//     start_url: &String,
// ) -> Result<DeviceAuthCredentials, ssooidc::Error> {
//     let get_device_authorization = client
//         .start_device_authorization()
//         .client_id(&device_credentials.client_id)
//         .client_secret(&device_credentials.client_secret)
//         .start_url(start_url)
//         .send()
//         .await?;
// 
//     Ok(DeviceAuthCredentials {
//         user_code: get_device_authorization.user_code.unwrap(),
//         device_code: get_device_authorization.device_code.unwrap(),
//         verification_url: get_device_authorization.verification_uri_complete.unwrap(),
//     })
// }
// 
// pub async fn generate_token(
//     client: &ssooidc::client::Client,
//     device_client_credentials: &DeviceClientCredentials,
//     device_auth_credentials: &DeviceAuthCredentials,
// ) -> Result<String, ssooidc::Error> {
//     // Generate the token, and return this token
//     let generate_token_output = client
//         .create_token()
//         .client_id(&device_client_credentials.client_id)
//         .client_secret(&device_client_credentials.client_secret)
//         .set_grant_type(Some(
//             "urn:ietf:params:oauth:grant-type:device_code".to_owned(),
//         ))
//         .device_code(&device_auth_credentials.device_code)
//         .code(&device_auth_credentials.user_code)
//         .send()
//         .await?;
// 
//     Ok(generate_token_output.access_token.unwrap())
// }
// 
// pub async fn get_account_list(
//     client: &sso::client::Client,
//     token: &String,
// ) -> Result<Vec<AccountInfo>, sso::Error> {
//     let account_list_output = client
//         .list_accounts()
//         .access_token(token)
//         .max_results(123)
//         .send()
//         .await?;
// 
//     Ok(account_list_output.account_list.unwrap())
// }
// 
// pub async fn get_account_credentials(
//     client: &sso::client::Client,
//     account_id: &String,
//     token: &String,
//     account_name: &String,
// ) -> Result<Vec<AccountCredentials>, sso::Error> {
//     let roles = client
//         .list_account_roles()
//         .account_id(account_id)
//         .access_token(token)
//         .send()
//         .await?;
// 
//     let mut account_credentials: Vec<AccountCredentials> = vec![];
// 
//     for role in roles.role_list.unwrap() {
//         let role_credentials = client
//             .get_role_credentials()
//             .role_name(role.role_name.clone().unwrap())
//             .account_id(account_id)
//             .access_token(token)
//             .send()
//             .await?;
// 
//         account_credentials.push(AccountCredentials {
//             account_name: account_name.replace(" ", ""),
//             role_name: role.role_name.clone().unwrap(),
//             aws_access_key_id: role_credentials
//                 .role_credentials
//                 .clone()
//                 .unwrap()
//                 .access_key_id
//                 .unwrap(),
//             aws_secret_access_key: role_credentials
//                 .role_credentials
//                 .clone()
//                 .unwrap()
//                 .secret_access_key
//                 .unwrap(),
//             aws_session_token: role_credentials
//                 .role_credentials
//                 .clone()
//                 .unwrap()
//                 .session_token
//                 .unwrap(),
//         })
//     }
// 
//     Ok(account_credentials)
// }

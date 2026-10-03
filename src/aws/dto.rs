pub struct DeviceClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

pub struct DeviceAuthCredentials {
    pub user_code: String,
    pub device_code: String,
    pub verification_url: String,
    pub interval: u64,
    pub expires_in: u64,
}

#[derive(Debug)]
pub struct AccountCredentials {
    pub account_name: String,
    pub role_name: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String,
}

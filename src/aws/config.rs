use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
use aws_sdk_sso::config::Region;

pub async fn init_config(region: impl Into<String>) -> SdkConfig {
    let region_provider = RegionProviderChain::first_try(Region::new(region.into()))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));

    aws_config::from_env().region(region_provider).load().await
}

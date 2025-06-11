use clap::Parser;
use console::style;

#[derive(Parser, Debug)]
#[clap(
    about = "aws-sso-rs",
    version = env!("CARGO_PKG_VERSION"),
    author = "containerscrew info@containerscrew.com",
    about = print_about(),
    arg_required_else_help = true,
    after_help = print_after_help_message(),
)]
pub struct Args {
    #[arg(
        short = 'l',
        long = "log-level",
        help = "Log level for logging tracing",
        default_value = "error",
        value_parser = ["info", "warn", "trace", "debug", "error"],
        required = false
    )]
    pub log_level: String,
    #[arg(
        short = 's',
        long = "start-url",
        help = "AWS start URL endpoint. Example: https://company.awsapps.com/start",
        required = true
    )]
    pub start_url: String,
    #[arg(
        short = 'r',
        long = "aws-region",
        help = "AWS region where you have configured SSO",
        required = true,
        default_value = "us-east-1"
    )]
    pub aws_region: String,
    #[arg(
        long = "role-overrides",
        help = "Override the role name in the profile ~/.aws/credentials. Default to [AccountName@RoleName]. Ej: --role-overrides role1=newrolename,role2=newrolename2",
        value_parser = parse_key_val_string,
        value_delimiter = ',',
        num_args = 1..,
        required = false
    )]
    pub role_overrides: Option<Vec<(String, String)>>,
    #[arg(
        long = "account-overrides",
        help = "Override the account name in the profile ~/.aws/credentials. Default to [AccountName@RoleName]. Ej: --account-overrides account1=newaccountname,account2=newaccountname2",
        value_parser = parse_key_val_string,
        value_delimiter = ',',
        num_args = 1..,
        required = false
    )]
    pub account_overrides: Option<Vec<(String, String)>>,
}

fn parse_key_val_string(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid format: expected key=value, got '{}'", s));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn print_about() -> String {
    format!(
        "{} v{} - Fetch your local ~/.aws/credentials using AWS SSO. \n",
        style("aws-sso-rs").bold().red(),
        env!("CARGO_PKG_VERSION"),
    )
}

fn print_after_help_message() -> String {
    format!("Author: containerscrew \nWebsite: github.com/containerscrew/aws-sso-rs\nLicense: GPL 3\nIssues: github.com/containerscrew/aws-sso-rs/issues")
}

use clap::Parser;
use current_platform::CURRENT_PLATFORM;

#[derive(Parser, Debug)]
#[clap(
    about = "aws-sso-rs",
    version = env!("CARGO_PKG_VERSION"),
    author = "containerscrew info@containerscrew.com",
    about = print_about(),
    arg_required_else_help = true
)]
pub struct Args {
    #[arg(
        short = 'l',
        long = "log-level",
        help = "Log level for logging tracing",
        default_value = "info",
        value_parser = ["info", "warn", "trace", "debug", "error"],
        required = false
    )]
    pub log_level: String,
    #[arg(
        short = 's',
        long = "start-url",
        help = "AWS start URL endpoint. Example: https://XXXXXX.awsapps.com/start",
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
}

fn print_about() -> String {
    format!(
        "aws-sso-rs v{} - Fetch your ~/.aws/credentials using AWS SSO and your external IDP \n Running on platform: {}",
        env!("CARGO_PKG_VERSION"),
        CURRENT_PLATFORM
    )
}

/*#[derive(Subcommand)]
enum Commands {
    /// Set your configuration for all your AWS SSO portals. Will be saved in ~/.aws/aws-sso-auth.json
    Config {
        #[arg(
            short = 's',
            long = "start-url",
            help = "AWS start URL endpoint. Example: https://XXXXXX.awsapps.com/start",
            required = true
        )]
        start_url: String,
        #[arg(
            short = 'r',
            long = "aws-region",
            help = "AWS region where you have configured SSO",
            required = true,
            default_value = "us-east-1"
        )]
        aws_region: String,
        #[arg(
            short = 'p',
            long = "profile-name",
            help = "The name with which you want to save the configuration. Your company name for example.",
            required = true
        )]
        profile_name: String,
    },
    /// Start fetching your AWS credentials
    Start {
        #[arg(
            short = 'w',
            long = "workers",
            help = "Number of threads! Recommended: 5/8 max to avoid AWS API 429 errors TooManyRequestsException",
            default_value = "6",
            required = false
        )]
        workers: usize,
        #[arg(
            short = 'r',
            long = "retries",
            help = "Number of retries when you have AWS API errors",
            default_value = "60",
            required = false
        )]
        retries: u32,
    },
}
*/
/*pub fn argparse() -> Result<Cli, Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Config {
            start_url,
            aws_region,
            profile_name,
        }) => {
            // Logging
            setup_logger(&cli.log_level);

            let config: Configuration = Configuration::new(
                start_url.to_string(),
                aws_region.to_string(),
                profile_name.to_string(),
            );

            // Write configuration
            config.write_config_file();
        }
        Some(Commands::Start { workers, retries }) => {
            // Logging
            setup_logger(&cli.log_level);

            // Read and deserialize data from config file
            let config_params = read_config_file();

            // Start AWS SDK API CALLS with tokio runtime builder
            start(
                config_params.parameters.aws_region,
                config_params.parameters.start_url,
                *workers,
                *retries,
            )
        }
        None => {}
    }

    // Return cli
    Ok(cli)
}*/

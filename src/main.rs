mod cli;
pub(crate) mod common;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::BymlToYaml::from_env_or_exit();
    return cli::Runner::new(cli).run();
}

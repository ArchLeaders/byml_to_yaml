mod cli;
pub(crate) mod common;

fn main() {
    let cli = cli::BymlToYaml::from_env_or_exit();
    match cli::Runner::new(cli).run() {
        Ok(()) => println!("Command executed successfully"),
        Err(error) => println!("Error executaing command: {error:?}"),
    }
}

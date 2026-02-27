use eyre::Result;

pub fn env_str(var_name: &str) -> Result<String> {
    std::env::var(var_name)
        .map_err(|_| eyre::eyre!("{} environment variable must be set", var_name))
}

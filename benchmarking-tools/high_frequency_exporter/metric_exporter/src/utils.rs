use eyre::Result;

pub fn env_str(var_name: &str) -> Result<String> {
    std::env::var(var_name)
        .map_err(|_| eyre::eyre!("{} environment variable must be set", var_name))
}

pub fn env_u16(var_name: &str) -> Result<u16> {
    let var_str = env_str(var_name)?;
    var_str.parse::<u16>().map_err(|_| {
        eyre::eyre!(
            "{} environment variable must be set and a valid u16",
            var_name
        )
    })
}

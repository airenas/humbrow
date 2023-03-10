use clap::ArgMatches;

pub struct Config {
    pub port: u16,
    pub version: String,
    pub python: String,
    pub cookie_script: String,
    pub agent_script: String,
}

impl Config {
    pub fn build(args: &ArgMatches) -> Result<Config, String> {
        let port = match args.get_one::<String>("port") {
            Some(v) => v
                .trim()
                .parse::<u16>()
                .map_err(|e| -> String { format!("can't parse port: `{v}`, {e}") }),
            None => Err(String::from("no port provided")),
        }?;
        Ok(Config {
            port,
            version: "dev".to_string(),
            python: option_env!("PYTHON").unwrap_or("ipython").into(),
            cookie_script: option_env!("COOKIE_SCRIPT")
                .unwrap_or("scripts/cookie.py")
                .into(),
            agent_script: option_env!("AGENT_SCRIPT")
                .unwrap_or("scripts/agent.py")
                .into(),
        })
    }
}

#[cfg(test)]
mod tests {
}

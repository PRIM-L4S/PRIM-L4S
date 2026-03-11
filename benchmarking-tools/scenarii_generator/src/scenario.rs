use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerConfig {
    sysctls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IperfConfig {
    bitrate: String,
}

impl Default for IperfConfig {
    fn default() -> Self {
        Self {
            bitrate: "0".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScenarioConfig {
    docker: DockerConfig,
    iperf: IperfConfig,
}

impl ScenarioConfig {
    pub fn from(cca_name: &str) -> Self {
        Self {
            docker: DockerConfig {
                sysctls: vec!["net.ipv4.tcp_congestion_control=".to_string() + cca_name],
            },
            iperf: IperfConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterConfig {
    max_bandwidth: String,
    delay_config: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scenario {
    description: String,
    configs: HashMap<String, ScenarioConfig>,
    instances: HashMap<String, String>,
    router: RouterConfig,
    template_generated_info: String,
}

impl Scenario {
    pub fn new(max_bandwidth: String, delay_config: String) -> Self {
        Self {
            description: "No clients.".into(),
            configs: HashMap::default(),
            instances: HashMap::default(),
            router: RouterConfig {
                max_bandwidth,
                delay_config,
            },
            template_generated_info: "This file is generated from a template. Do not edit directly. Instead, edit the template with a `.j2` extension.".into(),
        }
    }

    pub fn add_instances(&mut self, name: &str, config: ScenarioConfig, count: u32) {
        self.configs.entry(name.to_string()).insert_entry(config);

        for i in 0..count {
            self.instances
                .entry(name.to_string() + &i.to_string())
                .insert_entry(name.to_string());
        }
    }

    pub fn finalize(&mut self) {
        let mut description = String::new();

        if self.instances.len() == 0 {
            description += "No clients."
        } else {
            description += &self
                .instances
                .values()
                .counts()
                .into_iter()
                .map(|(name, count)| {
                    count.to_string() + " " + name + (if count > 1 { "s" } else { "" })
                })
                .join(", ");
            description += " sharing a common ";
            description += &self.router.max_bandwidth;
            description += " link.";
        }

        self.description = description;
    }

    // "n1reno+n2prague@bandwidth+mean_delay+delay.json"
    pub fn get_filename(&self) -> String {
        let bandwidth = &self.router.max_bandwidth;
        let delay = self.router.delay_config.replace(" ", "+");

        let clients = if self.instances.is_empty() {
            "no-clients"
        } else {
            &self
                .instances
                .values()
                .counts()
                .into_iter()
                .map(|(name, count)| count.to_string() + name)
                .sorted()
                .join("+")
        };

        format!("{}@{}+{}.json", clients, bandwidth, delay)
    }
}

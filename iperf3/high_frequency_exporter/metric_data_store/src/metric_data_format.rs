use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricDataFormat {
    metric: MetricLabels,

    timestamps: Vec<u128>,
    values: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MetricLabels {
    __name__: String,
    /// The name of the client
    host: String,
}

impl MetricDataFormat {
    pub fn new(metric_name: &str, host: &str) -> Self {
        MetricDataFormat {
            metric: MetricLabels {
                __name__: metric_name.to_string(),
                host: host.to_string(),
            },
            timestamps: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, timestamp: u128, value: u64) {
        if self.values.len() > 0 && self.values[self.values.len() - 1] == value {
            // Skip duplicate values to reduce data size
            return;
        }

        self.timestamps.push(timestamp);
        self.values.push(value);
    }

    pub fn clear(&mut self) {
        self.timestamps.clear();
        self.values.clear();
    }
}

pub trait MetricDataToImport {
    fn to_import_format(&self) -> String;

    fn clear(&mut self);
}

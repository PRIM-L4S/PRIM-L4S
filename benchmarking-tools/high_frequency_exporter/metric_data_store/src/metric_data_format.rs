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
    /// The congestion control algorithm in use, e.g. "cubic" or "prague"
    #[serde(skip_serializing_if = "Option::is_none")]
    congestion: Option<String>,
}

impl MetricDataFormat {
    pub fn new(metric_name: &str, host: &str, congestion: &Option<String>) -> Self {
        MetricDataFormat {
            metric: MetricLabels {
                __name__: metric_name.to_string(),
                host: host.to_string(),
                congestion: congestion.to_owned(),
            },
            timestamps: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, timestamp: u128, value: u64) {
        if self.values.last() == Some(&value) {
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

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

pub trait MetricDataToImport {
    fn to_import_format(&self) -> String;

    fn clear(&mut self);
}

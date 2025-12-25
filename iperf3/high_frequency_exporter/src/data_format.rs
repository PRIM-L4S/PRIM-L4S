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
    label_test1: String,
    label_test2: String,
}

impl MetricDataFormat {
    pub fn new(metric_name: &str, label1: &str, label2: &str) -> Self {
        MetricDataFormat {
            metric: MetricLabels {
                __name__: metric_name.to_string(),
                label_test1: label1.to_string(),
                label_test2: label2.to_string(),
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

        self.values.push(value);
        self.timestamps.push(timestamp);
    }
}

pub trait VictoriaMetricsFormat {
    fn to_import_format(&self) -> String;
}

impl VictoriaMetricsFormat for Vec<MetricDataFormat> {
    fn to_import_format(&self) -> String {
        let mut result = String::new();

        for metric_data in self {
            let data_json = serde_json::to_string(metric_data).unwrap();
            result.push_str(&data_json);
            result.push('\n');
        }

        result
    }
}

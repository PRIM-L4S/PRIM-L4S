use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::{DeriveInput, Field, parse_macro_input};

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

pub fn victoria_metrics_formatting_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let syn::Data::Struct(data) = &input.data else {
        abort!(input, "This derive macro can only be applied on structs");
    };

    // Validate fields type
    for field in &data.fields {
        let field_type = &field.ty;
        let is_metric_data_format = match field_type {
            syn::Type::Path(type_path) => type_path
                .path
                .segments
                .last()
                .map_or(false, |segment| segment.ident == "MetricDataFormat"),
            _ => false,
        };

        if !is_metric_data_format {
            abort!(field, "All fields must be of type MetricDataFormat");
        }
    }

    let new_fields = data.fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: MetricDataFormat::new(stringify!(#field_name), "value1", "value2"),
        }
    });

    let extract_data_from_fields = data.fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            let data_json = serde_json::to_string(&self.#field_name).unwrap();
            result.push_str(&data_json);
            result.push('\n');
        }
    });

    quote! {
    impl #ident {
        pub fn new() -> Self {
            #ident {
                #(#new_fields)*
            }
        }
    }

    impl VictoriaMetricsFormat for #ident {
        fn to_import_format(&self) -> String {
            let mut result = String::new();

            #(#extract_data_from_fields)*

            result
        }
    }}
    .into()
}

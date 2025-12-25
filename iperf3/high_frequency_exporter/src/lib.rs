use proc_macro::TokenStream;

mod data_format;
mod data_store;
mod generate_fake_data;
mod loop_gathering;
mod loop_sending;

#[proc_macro_derive(VictoriaMetricsFormatting)]
/**
 * Implements the VictoriaMetricsFormat trait for a struct of MetricDataFormat
 */
pub fn victoria_metrics_formatting_derive(input: TokenStream) -> TokenStream {
    data_format::victoria_metrics_formatting_derive(input)
}

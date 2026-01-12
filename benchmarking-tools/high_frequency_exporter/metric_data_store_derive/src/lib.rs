use proc_macro::TokenStream;

mod derive_import_format;

#[proc_macro_derive(ToImportFormat)]
/**
 * Implements the new method and the MetricDataToImport trait for a struct of MetricDataFormat
 */
pub fn to_import_format_derive(input: TokenStream) -> TokenStream {
    derive_import_format::to_import_format_derive(input)
}

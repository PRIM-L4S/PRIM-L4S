use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn to_import_format_derive(input: TokenStream) -> TokenStream {
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
            #field_name: MetricDataFormat::new(stringify!(#field_name), "todo"),
        }
    });

    let extract_data_from_fields = data.fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            let data_json = ::serde_json::to_string(&self.#field_name).unwrap();
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

    impl ::metric_data_store::MetricDataToImport for #ident {
        fn to_import_format(&self) -> String {
            let mut result = String::new();

            #(#extract_data_from_fields)*

            result
        }
    }}
    .into()
}

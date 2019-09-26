extern crate proc_macro;

use proc_macro::TokenStream;

use syn;

use quote::quote;

#[proc_macro_derive(Example, attributes(example_value))]
pub fn example_derive(token_stream: TokenStream) -> TokenStream {
    let ast = syn::parse(token_stream).unwrap();
    impl_example_macro(&ast)
}

fn impl_example_macro(input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    let data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("examples can only be derived for structs"),
    };

    let fields: Vec<_> = data
        .fields
        .iter()
        .map(|field| {
            (
                field.ident.as_ref().expect("field should have and ident"),
                field
                    .attrs
                    .iter()
                    .find(|attr| attr.path.get_ident().map(|it| it.to_string()) == Some("example_value".into())),
            )
        })
        .map(|(ident, value)| {
            if let Some(value) = value {
                let value = &value.tokens;
                quote!(#ident: #value.into())
            } else {
                quote!(#ident: hermes_utils::Example::example(hermes_utils::ExampleConfig {
                    field_name: Some(stringify!(#ident).into()),
                    .. config.clone()
                }))
            }
        })
        .collect();

    quote!(
        impl hermes_utils::Example for # struct_name {
            fn example(config: hermes_utils::ExampleConfig) -> Self {
                Self {
                    # ( # fields, )*
                }
            }
        }
    )
    .into()
}

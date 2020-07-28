use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Fields};

// Getter derive macro
#[proc_macro_derive(Getter)]
pub fn getter_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let generics = ast.generics.clone();

    let struct_name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Expected a struct with named fields."),
    };

    let field_names = fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
    let field_types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();

    let getter_names = field_names
        .iter()
        .map(|field_name| {
            format_ident!(
                "get_{}",
                field_name
                    .as_ref()
                    .unwrap_or_else(|| panic!("Unable to parse identifier."))
            )
        })
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        impl #generics #struct_name #generics {
            #( pub fn #getter_names (&self) -> #field_types {
                self.#field_names.clone()
            } )*
        }
    })
}

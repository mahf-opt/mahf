use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field,
    Fields, GenericParam,
};

fn parse_fields(input: &DeriveInput) -> Punctuated<Field, Comma> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.clone(),
            Fields::Unnamed(_) => panic!("Tuple structs are not supported by this macro"),
            Fields::Unit => Punctuated::new(),
        },
        _ => panic!("This macro only works on structs"),
    }
}

/// Implements `mahf::params::TryFromParams`.
#[proc_macro_derive(TryFromParams)]
pub fn from_params_derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    // Parse struct name.
    let name = &input.ident;

    // Parse field name and type.
    let fields = parse_fields(&input);
    let (keys, types): (Vec<_>, Vec<_>) = fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|ident| (ident, &field.ty)))
        .unzip();

    // Parse generics and trait bounds.
    for param in &mut input.generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(parse_quote!(::mahf::params::Parameter))
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Implement `TryFrom<Param>` for #name using the field #keys and #types.
    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::std::convert::TryFrom<::mahf::params::Params> for #name #ty_generics #where_clause {
            type Error = ::eyre::Report;

            fn try_from(mut params: ::mahf::params::Params) -> Result<Self, Self::Error> {
                #(
                    let Some(#keys) = params.extract::<#types>(stringify!(#keys)) else { ::eyre::bail!("missing {}", stringify!(#keys)) };
                )*
                Ok(Self { #(#keys),* })
            }
        }
    };

    expanded.into()
}

/// Implements `mahf::params::Parametrized`.
#[proc_macro_derive(Parametrized)]
pub fn parametrized_derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    // Parse struct name.
    let name = &input.ident;

    // Parse field names.
    let fields = parse_fields(&input);
    let keys: Vec<_> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect();

    // Parse generics and trait bounds.
    for param in &mut input.generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(parse_quote!(::mahf::params::Parameter));
            type_param.bounds.push(parse_quote!(::std::clone::Clone));
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Implement `TryFrom<Param>` for #name using the field #keys and #types.
    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::mahf::params::Parametrized for #name #ty_generics #where_clause {
            fn param_names(&self) -> ::std::collections::HashSet<::std::borrow::Cow<str>> {
                [#(stringify!(#keys)),*]
                    .into_iter()
                    .map(|key: &str| ::std::borrow::Cow::from(key))
                    .collect()
            }

            #[allow(clippy::clone_on_copy)]
            fn get_params(&self) -> ::mahf::params::Params {
                ::mahf::params::Params::new()
                #(
                    .with(stringify!(#keys), self.#keys.clone())
                )*
            }

            fn set_params(&mut self, mut params: ::mahf::params::Params) {
                #(
                    if let Some(value) = params.extract(stringify!(#keys)) {
                        self.#keys = value;
                    }
                )*
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(CustomState)]
pub fn custom_state_derive(_input: TokenStream) -> TokenStream {
    todo!()
}

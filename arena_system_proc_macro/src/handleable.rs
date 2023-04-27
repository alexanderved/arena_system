use crate::util::iter_generics;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_quote, punctuated::Punctuated, Data, DeriveInput, Field, Fields, GenericParam, Generics,
    Ident, Lifetime, Token, Type, Visibility,
};

#[derive(Debug, Clone)]
pub struct HandleableInfo {
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Punctuated<Field, Token![,]>,

    pub handle_ident: Ident,
    pub lifetime: Lifetime,
}

impl HandleableInfo {
    pub fn parse(input: syn::DeriveInput) -> Self {
        let DeriveInput { vis, ident, mut generics, data, .. } = input;

        let fields = match data {
            Data::Struct(struct_data) => match struct_data.fields {
                Fields::Named(fields_named) => fields_named.named,
                Fields::Unnamed(_) => {
                    unimplemented!("Structs with unnamed fields are not supported")
                }
                Fields::Unit => unimplemented!("Unit structs are not supported"),
            },
            Data::Enum(_) => unimplemented!("Enums are not supported"),
            Data::Union(_) => unimplemented!("Unions are not supported"),
        };

        let lifetime = Lifetime::new("'arena", Span::call_site());
        generics.params.iter_mut().for_each(|g| {
            if let GenericParam::Type(ref mut t) = g {
                t.bounds.push(syn::TypeParamBound::Lifetime(lifetime.clone()));
            }
        });

        let handle_ident = format_ident!("{}Handle", ident);

        Self { vis, ident, generics, fields, handle_ident, lifetime }
    }

    pub fn quote_impl(&self) -> TokenStream {
        let HandleableInfo { generics, handle_ident, lifetime, .. } = self;

        let handleable_type = self.to_type();
        let (impl_generics, generics_types, where_clause) = iter_generics(generics);

        quote! {
            impl<#lifetime, #( #impl_generics ),*> arena_system::Handleable<#lifetime>
                for #handleable_type #where_clause
            {
                type Handle = #handle_ident <#lifetime, #( #generics_types ),*>;
            }
        }
    }

    pub fn to_type(&self) -> Type {
        let HandleableInfo { ident, generics, .. } = self;

        let (_, ty_generics, _) = generics.split_for_impl();

        parse_quote!(#ident #ty_generics)
    }
}

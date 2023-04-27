extern crate proc_macro;

mod util;
mod getter;

use util::*;
use getter::*;

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::Result, parse_macro_input, parse_quote, punctuated::Punctuated,
    Data, DeriveInput, Field, Fields, GenericParam, Generics, Ident, Lifetime,
    Token, Type, Visibility,
};

#[derive(Debug, Clone)]
struct HandleableInfo {
    vis: Visibility,
    ident: Ident,
    lifetime: Lifetime,
    generics: Generics,
    fields: Punctuated<Field, Token![,]>,
}

impl HandleableInfo {
    fn parse(input: syn::DeriveInput) -> Self {
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

        Self { vis, ident, lifetime, generics, fields }
    }

    fn to_type(&self) -> Type {
        let HandleableInfo { ident, generics, .. } = self;

        let (_, ty_generics, _) = generics.split_for_impl();

        parse_quote!(#ident #ty_generics)
    }

    fn quote_impl(&self) -> TokenStream {
        let HandleableInfo { ident, generics, lifetime, .. } = self;

        let handleable_type = self.to_type();

        let where_clause = &generics.where_clause;
        let impl_generics = generics.params.iter();

        let generics_types = impl_generics.clone().map(|g| match g {
            GenericParam::Type(t) => &t.ident,
            GenericParam::Const(c) => &c.ident,
            GenericParam::Lifetime(_) => unimplemented!(),
        });

        let handle_ident = format_ident!("{}Handle", ident);

        quote! {
            impl<#lifetime, #( #impl_generics ),*> arena_system::Handleable<#lifetime>
                for #handleable_type #where_clause
            {
                type Handle = #handle_ident <#lifetime, #( #generics_types ),*>;
            }
        }
    }
}

struct HandleInfo<'a> {
    handleable: &'a HandleableInfo,

    vis: Visibility,
    ident: Ident,
    #[allow(unused)]
    userdata: Option<HashMap<Ident, Type>>,
}

impl<'a> HandleInfo<'a> {
    fn parse(handleable_info: &'a HandleableInfo) -> Self {
        let HandleableInfo { vis, ident, .. } = handleable_info;

        let handle_ident = format_ident!("{}Handle", ident);

        Self { handleable: handleable_info, vis: vis.clone(), ident: handle_ident, userdata: None }
    }

    fn to_type(&self) -> Type {
        let HandleInfo { handleable, ident, .. } = self;

        let lifetime = &handleable.lifetime;
        let (_, ty_generics, _) = iter_generics(&handleable.generics);

        parse_quote!(#ident < #lifetime, #( #ty_generics ),* >)
    }

    fn quote(self) -> Result<TokenStream> {
        let HandleInfo { handleable, .. } = self;

        let handle_decl = self.handle_decl();

        let lifetime = &handleable.lifetime;
        let (impl_generics, _, where_clause) = iter_generics(&handleable.generics);
        let handleable_type = handleable.to_type();
        let handle_type = self.to_type();

        let getters = self.getters()?;

        Ok(quote! {
            #handle_decl

            impl<#lifetime, #( #impl_generics ),*> arena_system::Handle<#lifetime>
                for #handle_type #where_clause
            {
                type Type = #handleable_type;
                type Userdata = arena_system::EmptyUserdata;

                fn from_raw(
                    raw: arena_system::RawHandle<#lifetime, Self::Type>,
                    userdata: Self::Userdata
                ) -> Self {
                    Self {
                        __raw: raw,
                    }
                }

                fn to_raw(&self) -> arena_system::RawHandle<#lifetime, Self::Type> {
                    self.__raw
                }
            }

            #getters
        })
    }

    fn handle_decl(&self) -> TokenStream {
        let HandleInfo { vis, ident, handleable, .. } = self;

        let lifetime = &handleable.lifetime;
        let handleable_generics_params = handleable.generics.params.iter();

        let handleable_type = handleable.to_type();
        let where_clause = &handleable.generics.where_clause;

        quote! {
            #vis struct #ident <#lifetime, #( #handleable_generics_params ),*> #where_clause {
                __raw: arena_system::RawHandle<#lifetime, #handleable_type>,
            }
        }
    }

    fn getters(&self) -> Result<TokenStream> {
        let lifetime = &self.handleable.lifetime;
        let (impl_generics, _, where_clause) = iter_generics(&self.handleable.generics);
        let handle_type = self.to_type();

        let getters = self
            .handleable
            .fields
            .iter()
            .map(|f| {
                let getter = Getter::new(f)?;

                Ok(getter.quote())
            })
            .collect::<Result<Vec<_>>>();

        getters.map(|getters| {
            quote! {
                impl<#lifetime, #( #impl_generics ),*> #handle_type #where_clause {
                    #( #getters )*
                }
            }
        })
    }
}

#[proc_macro_derive(Handleable, attributes(handle_getter))]
pub fn derive_handleable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let handleable_info = HandleableInfo::parse(input);
    let handle_info = HandleInfo::parse(&handleable_info);

    let handleable_impl = handleable_info.quote_impl();
    let handle = match handle_info.quote() {
        Ok(h) => h,
        Err(err) => return err.to_compile_error().into(),
    };

    quote! {
        #handleable_impl

        #handle
    }
    .into()
}

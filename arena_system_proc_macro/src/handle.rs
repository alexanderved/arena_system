use crate::getter::*;
use crate::handleable::*;
use crate::util::*;

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Result, parse_quote, Ident, Type, Visibility};

pub struct HandleInfo<'a> {
    pub handleable: &'a HandleableInfo,

    pub vis: &'a Visibility,
    pub ident: &'a Ident,
    #[allow(unused)]
    pub userdata: Option<HashMap<Ident, Type>>,
}

impl<'a> HandleInfo<'a> {
    pub fn parse(handleable_info: &'a HandleableInfo) -> Self {
        Self {
            handleable: handleable_info,
            vis: &handleable_info.vis,
            ident: &handleable_info.handle_ident, 
            userdata: None,
        }
    }

    pub fn to_type(&self) -> Type {
        let HandleInfo { handleable, ident, .. } = self;

        let lifetime = &handleable.lifetime;
        let (_, ty_generics, _) = iter_generics(&handleable.generics);

        parse_quote!(#ident < #lifetime, #( #ty_generics ),* >)
    }

    pub fn quote(self) -> Result<TokenStream> {
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

    pub fn handle_decl(&self) -> TokenStream {
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

    pub fn getters(&self) -> Result<TokenStream> {
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
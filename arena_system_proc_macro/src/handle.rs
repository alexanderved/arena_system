use crate::getter::Getter;
use crate::setter::Setter;
use crate::handleable::HandleableInfo;
use crate::util::iter_generics;

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

    pub fn quote(self) -> Result<TokenStream> {
        let handle_decl = self.handle_decl();
        let handle_impl = self.handle_impl();
        let getters = self.getters()?;
        let setters = self.setters()?;

        Ok(quote! {
            #handle_decl

            #handle_impl

            #getters

            #setters
        })
    }

    fn to_type(&self) -> Type {
        let HandleInfo { handleable, ident, .. } = self;

        let lifetime = &handleable.lifetime;
        let (_, ty_generics, _) = iter_generics(&handleable.generics);

        parse_quote!(#ident < #lifetime, #( #ty_generics ),* >)
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
                tests: arena_system::Arena<Test<24, u32>>,
            }
        }
    }

    fn handle_impl(&self) -> TokenStream {
        let lifetime = &self.handleable.lifetime;
        let (impl_generics, _, where_clause) = iter_generics(&self.handleable.generics);
        let handleable_type = self.handleable.to_type();
        let handle_type = self.to_type();

        quote! {
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
                        tests: arena_system::Arena::new(),
                    }
                }

                fn to_raw(&self) -> arena_system::RawHandle<#lifetime, Self::Type> {
                    self.__raw
                }
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
                let getter = Getter::new(f, lifetime)?;

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

    fn setters(&self) -> Result<TokenStream> {
        let lifetime = &self.handleable.lifetime;
        let (impl_generics, _, where_clause) = iter_generics(&self.handleable.generics);
        let handle_type = self.to_type();

        let setters = self
            .handleable
            .fields
            .iter()
            .map(|f| {
                let setter = Setter::new(f)?;

                Ok(setter.quote())
            })
            .collect::<Result<Vec<_>>>();

        setters.map(|setters| {
            quote! {
                impl<#lifetime, #( #impl_generics ),*> #handle_type #where_clause {
                    #( #setters )*
                }
            }
        })
    }
}

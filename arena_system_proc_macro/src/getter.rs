use crate::util::{parse_name_attr, parse_vis_attr, unwrap_type};

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parenthesized, parse::Result, parse_quote, spanned::Spanned, Field, Ident, Lifetime, Type,
    Visibility, Token,
};

pub struct Getter {
    pub vis: Visibility,
    pub ident: Ident,
    pub return_ty: Type,
    pub body: TokenStream,
}

impl Getter {
    pub fn new(f: &Field, lifetime: &Lifetime) -> Result<Getter> {
        let field_ident = f.ident.clone().expect("Structs with unnamed fields are not supported");
        let field_ty = &f.ty;
        let field_ty_span = field_ty.span();

        let mut return_ty: Type =
            parse_quote!(Option<arena_system::ElementRef<#lifetime, #field_ty>>);
        let mut fn_body = quote_spanned! { field_ty_span =>
            use arena_system::Handle;
            self.get()
                .ok()
                .map(|this_ref| arena_system::ElementRef::map(
                    this_ref,
                    |this| {
                        &this.#field_ident
                    }
                ))
        };

        let mut fn_ident = field_ident.clone();
        let mut fn_vis = f.vis.clone();

        f.attrs
            .iter()
            .filter(|a| a.path().is_ident("handle_getter"))
            .try_for_each(|a| {
                a.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        fn_ident = parse_name_attr(meta)?;

                        return Ok(());
                    }

                    if meta.path.is_ident("vis") {
                        fn_vis = parse_vis_attr(meta)?;

                        return Ok(());
                    }

                    if meta.path.is_ident("return_type") {
                        let return_type;
                        parenthesized!(return_type in meta.input);

                        let return_ident = return_type.parse::<Ident>()?;
                        match return_ident.to_string().as_str() {
                            "reference" => {
                                return_ty = parse_quote!(
                                    Option<arena_system::ElementRef<#lifetime, #field_ty>>
                                );
                                fn_body = quote_spanned! { field_ty_span =>
                                    use arena_system::Handle;
                                    self.get()
                                        .ok()
                                        .map(|this_ref| arena_system::ElementRef::map(
                                            this_ref,
                                            |this| &this.#field_ident,
                                        ))
                                };
                            }
                            "clone" => {
                                return_ty = parse_quote!(Option<#field_ty>);
                                fn_body = quote_spanned! { field_ty_span =>
                                    fn _static_assert_clone<_StaticAssertClone: Clone>() {}
                                    _static_assert_clone::<#field_ty>();

                                    use arena_system::Handle;
                                    self.get()
                                        .ok()
                                        .map(|this_ref| this_ref.#field_ident.clone())
                                };
                            }
                            "copy" => {
                                return_ty = parse_quote!(Option<#field_ty>);
                                fn_body = quote_spanned! { field_ty_span =>
                                    fn _static_assert_copy<_StaticAssertCopy: Copy>() {}
                                    _static_assert_copy::<#field_ty>();

                                    use arena_system::Handle;
                                    self.get()
                                        .ok()
                                        .map(|this_ref| this_ref.#field_ident)
                                };
                            }
                            "handle" => {
                                let arena;
                                parenthesized!(arena in return_type);

                                let arena_ident = arena.parse::<Ident>()?;
                                arena.parse::<Token![:]>()?;
                                let arena_type = arena.parse::<Type>()?;
                                let element_type = unwrap_type("Arena", &arena_type)?;

                                return_ty = parse_quote!(
                                    Option<
                                        <#element_type as arena_system::Handleable<'arena>>::Handle
                                    >
                                );
                                fn_body = quote_spanned! { field_ty_span =>
                                    use arena_system::Handle;

                                    self.get()
                                        .ok()
                                        .map(|this_ref| {
                                            self.#arena_ident
                                                .handle(this_ref.#field_ident.into(), None)
                                        })
                                };
                            }
                            _ => return Err(meta.error("unrecognised return type")),
                        }

                        return Ok(());
                    }

                    Err(meta.error("unrecognised getter attribute"))
                })
            })?;

        Ok(Getter { vis: fn_vis, ident: fn_ident, return_ty, body: fn_body })
    }

    pub fn quote(self) -> TokenStream {
        let Getter { vis, ident, return_ty, body } = self;

        quote! {
            #vis fn #ident(&'arena self) -> #return_ty {
                #body
            }
        }
    }
}

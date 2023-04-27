use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parenthesized, parse::Result, parse_quote, spanned::Spanned, Field, Ident, Lifetime, Token,
    Type, VisRestricted, Visibility,
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
                        let name;
                        parenthesized!(name in meta.input);
                        fn_ident = name.parse()?;

                        return Ok(());
                    }

                    if meta.path.is_ident("vis") {
                        let vis;
                        parenthesized!(vis in meta.input);

                        if vis.peek(Token![priv]) {
                            vis.parse::<Token![priv]>()?;
                            fn_vis = Visibility::Inherited;
                        } else if vis.peek(Token![pub]) {
                            if vis.peek2(syn::token::Paren) {
                                let path;

                                fn_vis = Visibility::Restricted(VisRestricted {
                                    pub_token: vis.parse::<Token![pub]>()?,
                                    paren_token: parenthesized!(path in vis),
                                    in_token: path.parse::<Token![in]>().ok(),
                                    path: path.parse::<Box<syn::Path>>()?,
                                });
                            } else {
                                let pub_token = vis.parse::<Token![pub]>()?;
                                fn_vis = Visibility::Public(pub_token);
                            }
                        } else {
                            return Err(meta.error("unrecognised visiility level"));
                        }

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
            #vis fn #ident(&self) -> #return_ty {
                #body
            }
        }
    }
}

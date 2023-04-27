use crate::util::{parse_name_attr, parse_vis_attr};

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::{
    parenthesized, parse::Result, spanned::Spanned, Field, Ident, Type,
    Visibility,
};

pub struct Setter {
    pub vis: Visibility,
    pub ident: Ident,
    pub input_ty: Type,
    pub body: TokenStream,
}

impl Setter {
    pub fn new(f: &Field) -> Result<Setter> {
        let field_ident = f.ident.clone().expect("Structs with unnamed fields are not supported");
        let field_ty = &f.ty;
        let field_ty_span = field_ty.span();

        let mut input_ty: Type = field_ty.clone();
        let mut fn_body = quote_spanned! { field_ty_span =>
            use arena_system::Handle;
            self.get_mut()
                .map(|mut this_ref| {
                    this_ref.#field_ident = value;
                })
                .is_ok()
        };

        let mut fn_ident = format_ident!("set_{}", field_ident);
        let mut fn_vis = f.vis.clone();

        f.attrs
            .iter()
            .filter(|a| a.path().is_ident("handle_setter"))
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

                    if meta.path.is_ident("input_type") {
                        let input_type;
                        parenthesized!(input_type in meta.input);

                        let return_ident = input_type.parse::<Ident>()?;
                        match return_ident.to_string().as_str() {
                            "value" => {
                                input_ty = field_ty.clone();
                                fn_body = quote_spanned! { field_ty_span =>
                                    use arena_system::Handle;
                                    self.get_mut()
                                        .map(|mut this_ref| {
                                            this_ref.#field_ident = value;
                                        })
                                        .is_ok()
                                };
                            }
                            _ => return Err(meta.error("unrecognised input type")),
                        }

                        return Ok(());
                    }

                    Err(meta.error("unrecognised setter attribute"))
                })
            })?;

        Ok(Setter { vis: fn_vis, ident: fn_ident, input_ty, body: fn_body })
    }

    pub fn quote(self) -> TokenStream {
        let Setter { vis, ident, input_ty, body } = self;

        quote! {
            #vis fn #ident(&self, value: #input_ty) -> bool {
                #body
            }
        }
    }
}
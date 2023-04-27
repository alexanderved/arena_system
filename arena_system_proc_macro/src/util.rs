use syn::{
    meta::ParseNestedMeta, parenthesized, parse::Result, GenericParam, Generics, Ident, Token,
    VisRestricted, Visibility, WhereClause,
};

pub fn iter_generics(
    generics: &Generics,
) -> (std::vec::IntoIter<&GenericParam>, std::vec::IntoIter<proc_macro2::Ident>, Option<WhereClause>)
{
    let impl_generics = generics.params.iter().collect::<Vec<_>>();
    let ty_generics = impl_generics
        .iter()
        .map(|g| match g {
            GenericParam::Type(t) => t.ident.clone(),
            GenericParam::Const(c) => c.ident.clone(),
            GenericParam::Lifetime(_l) => unimplemented!(),
        })
        .collect::<Vec<_>>();
    let where_clause = generics.where_clause.clone();

    (impl_generics.into_iter(), ty_generics.into_iter(), where_clause)
}

pub fn parse_name_attr(meta: ParseNestedMeta) -> Result<Ident> {
    if meta.path.is_ident("name") {
        let name;
        parenthesized!(name in meta.input);
        return name.parse();
    }

    Err(meta.error("the given attribute isn't name"))
}

pub fn parse_vis_attr(meta: ParseNestedMeta) -> Result<Visibility> {
    if meta.path.is_ident("vis") {
        let vis;
        parenthesized!(vis in meta.input);

        if vis.peek(Token![priv]) {
            vis.parse::<Token![priv]>()?;
            return Ok(Visibility::Inherited);
        } else if vis.peek(Token![pub]) {
            if vis.peek2(syn::token::Paren) {
                let path;

                return Ok(Visibility::Restricted(VisRestricted {
                    pub_token: vis.parse::<Token![pub]>()?,
                    paren_token: parenthesized!(path in vis),
                    in_token: path.parse::<Token![in]>().ok(),
                    path: path.parse::<Box<syn::Path>>()?,
                }));
            } else {
                let pub_token = vis.parse::<Token![pub]>()?;
                return Ok(Visibility::Public(pub_token));
            }
        } else {
            return Err(meta.error("unrecognised visiility level"));
        }
    }

    Err(meta.error("the given attribute isn't visibility"))
}

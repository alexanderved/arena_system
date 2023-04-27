use syn::{Generics, GenericParam, WhereClause};

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
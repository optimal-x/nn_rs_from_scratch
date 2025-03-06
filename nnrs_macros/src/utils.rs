use std::rc::Rc;

use proc_macro2::Ident;
use proc_macro2::Span;

use syn::Path;
use syn::PathSegment;
use syn::Type;
use syn::TypePath;

/// type_lowercasification
/// ------
/// This basically uses the concrete types created path (eg. std::rs::Rc)
/// modifies its named identifier to lowercase. Updates the identifier on the
/// new_path and returns the type with the new ident.
pub(crate) fn type_lowercasification(ty: &Type) -> Type {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(ident) = path.get_ident() {
            let lower_ident = Ident::new(
                &ident.to_string().to_lowercase(),
                Span::call_site(),
            );

            let new_path = Path {
                leading_colon: path.leading_colon,
                segments: vec![PathSegment {
                    ident: lower_ident,
                    arguments: Default::default(),
                }]
                .into_iter()
                .collect(),
            };

            return Type::Path(TypePath {
                qself: None,
                path: new_path,
            });
        }
    }
    ty.clone()
}

/// self_inner_join
/// --------
/// Basically the same a self inner join in SQL. Or more broadly known as
/// a cartesian product with the same set.
pub(crate) fn self_inner_join<T>(types_arr: &Vec<T>) -> Vec<Vec<(&T, &T)>> {
    let types_rc = Rc::new(types_arr);
    let row = types_rc.clone().into_iter();

    row.enumerate()
        .map(|(i, T)| {
            // let col = row.take(i).chain(row.skip(i + 1));
            let col = types_rc.clone().into_iter().take(i);
            let to_skip = col.clone().skip(i + 1);
            let skipped = col.chain(to_skip);

            skipped.map(move |U| (T, U)).collect()
        })
        .collect()
}

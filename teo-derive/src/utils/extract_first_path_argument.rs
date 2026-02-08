use syn::{Error, GenericArgument, Path, PathArguments, Result, Type, spanned::Spanned};

pub(crate) fn extract_first_path_argument(path: &Path) -> Result<&Type> {
    if let Some(segment) = path.segments.last() {
        if let PathArguments::AngleBracketed(args) = &segment.arguments {
            if let Some(first) = args.args.first() {
                if let GenericArgument::Type(ty) = first {
                    Ok(ty)
                } else {
                    Err(Error::new(path.span(), "Invalid type."))
                }
            } else {
                Err(Error::new(path.span(), "Invalid type."))
            }
        } else {
            Err(Error::new(path.span(), "Invalid type."))
        }
    } else {
        Err(Error::new(path.span(), "Invalid type."))
    }
}

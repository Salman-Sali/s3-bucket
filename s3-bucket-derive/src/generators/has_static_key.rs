use quote::quote;

use crate::{StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_static_key(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();

    let Some(key) = &struct_info.key else {
        return quote! {};
    };

    if !key.is_static_key() {
        return quote! {};
    }

    let key_value = &key.value;
    quote! {
        impl s3_bucket::traits::has_static_key::HasStaticKey for #struct_name_expr {
            fn get_static_key() -> String {
                String::from(#key_value)
            }
        }
    }
}

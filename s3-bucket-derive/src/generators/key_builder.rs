use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_key_buidler(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();
    let Some(key) = &struct_info.key else {
        return quote! {};
    };

    if key.is_static_key() {
        return quote! {};
    }

    let value = key.value.replace(&format!("{{{}}}", key.argument.as_ref().unwrap()), "{value}");

    quote! {
        impl s3_bucket::traits::key_builder::KeyBuilder for #struct_name_expr {
            fn build_key(value: &String) -> String {
                format!(#value)
            }
        }
    }
}
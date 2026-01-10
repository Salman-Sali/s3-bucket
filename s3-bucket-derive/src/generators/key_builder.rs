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

    let mut key_string = key.value.clone();
    for argument in &key.arguments {
        key_string = key_string.replace(&format!("{{{}}}", argument), "{}");
    }

    quote! {
        impl s3_bucket::traits::key_builder::KeyBuilder for #struct_name_expr {
            fn build_key(arguments: Vec<Box<dyn std::fmt::Display + Send>>) -> String {
                let mut key_string = String::from(#key_string);
                arguments
                    .iter()
                    .fold(key_string, |acc, v| acc.replacen("{}", &v.to_string(), 1))
            }
        }
    }
}

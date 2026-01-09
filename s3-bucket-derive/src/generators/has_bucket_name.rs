use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_bucket_name_tokens(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let Some(bucket_name) = &struct_info.bucket else {
        return quote! {};
    };

    let struct_name_expr = struct_info.struct_name.as_expr();
    let bucket_name_expr = bucket_name.as_expr();
    quote! {
        impl s3_bucket::traits::has_bucket_name::HasBucketName for #struct_name_expr {
            fn get_bucket_name() -> String {
                #bucket_name_expr.clone()
            }
        }
    }
}
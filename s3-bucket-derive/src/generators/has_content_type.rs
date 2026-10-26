use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_content_type(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let Some(content_type) = &struct_info.content_type else {
        return quote! {};
    };
    let struct_name_expr = struct_info.struct_name.as_expr();
    let cache_control = struct_info.cache_control.as_ref().map(|cache_control| {
        quote! {
            fn get_cache_control() -> Option<String> {
                Some(String::from(#cache_control))
            }
        }
    });
    quote! {
        impl s3_bucket::traits::has_content_type::HasContentType for #struct_name_expr {
            fn get_content_type() -> String {
                String::from(#content_type)
            }

            #cache_control
        }
    }
}

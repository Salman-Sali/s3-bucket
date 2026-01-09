use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_key_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();

    let Some(key) = &struct_info.key else {
        return quote! {};
    };

    let key_value = &key.value;
    let key_token = if let Some(argument) = &key.argument {
        let argument_expr = argument.as_expr();
        quote! {
            let #argument_expr = &self.#argument_expr;
            #struct_name_expr::build_key(#argument_expr)
        }
    } else {
        quote! {String::from(#key_value)}
    };

    quote! {
        impl s3_bucket::traits::has_key::HasKey for #struct_name_expr {
            fn get_key(&self) -> String {
                use s3_bucket::traits::key_builder::KeyBuilder;
                #key_token
            }
        }
    }
}

use quote::{ToTokens, quote};

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_has_key_token(struct_info: &StructInfo) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();

    let Some(key) = &struct_info.key else {
        return quote! {};
    };
    let key_value = &key.value;
    let key_token = if key.is_static_key() {
        quote! {String::from(#key_value)}
    } else {
        let mut build_key_expr = quote! {
            let mut arguments: Vec<Box<dyn std::fmt::Display>> = vec![];
        };

        for argument in &key.arguments {
            let argument_expr = argument.as_expr();
            quote! {
                arguments.push(Box::new(self.#argument_expr.clone()));
            }
            .to_tokens(&mut build_key_expr);
        }
        quote! {
            #struct_name_expr::build_key(arguments)
        }
        .to_tokens(&mut build_key_expr);
        build_key_expr
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

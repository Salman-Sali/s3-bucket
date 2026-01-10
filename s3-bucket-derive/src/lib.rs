#![deny(unused_crate_dependencies)]

use generators::{
    has_bucket_name::generate_has_bucket_name_tokens, has_content_type::generate_has_content_type,
    has_key::generate_has_key_token, json_item::generate_byte_stream_conversion_for_json_item,
    key_builder::generate_key_buidler,
};
use proc_macro::TokenStream;
use quote::quote;
use struct_info::StructInfo;
use syn::{DeriveInput, parse_macro_input};

mod generators;
mod struct_info;
mod utils;

#[proc_macro_derive(S3BucketItem, attributes(s3_item_prop))]
pub fn s3_bucket_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_info = StructInfo::from(input);

    let has_bucket_name_token = generate_has_bucket_name_tokens(&struct_info);
    let has_content_type_token = generate_has_content_type(&struct_info);
    let has_key_token = generate_has_key_token(&struct_info);
    let key_builder_token = generate_key_buidler(&struct_info);

    quote! {
        #has_bucket_name_token
        #has_content_type_token
        #has_key_token
        #key_builder_token
    }
    .into()
}

#[proc_macro_derive(JsonItem)]
pub fn json_item_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_info = StructInfo::from(input);
    let conversion = generate_byte_stream_conversion_for_json_item(&struct_info);
    quote! {
        #conversion
    }
    .into()
}

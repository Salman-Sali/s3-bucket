use quote::quote;

use crate::{struct_info::StructInfo, utils::as_expr::AsExpr};

pub fn generate_byte_stream_conversion_for_json_item(
    struct_info: &StructInfo,
) -> proc_macro2::TokenStream {
    let struct_name_expr = struct_info.struct_name.as_expr();
    quote! {
        impl TryInto<s3_bucket::bytes::Bytes> for #struct_name_expr {
            type Error = s3_bucket::error::Error;

            fn try_into(self) -> Result<s3_bucket::bytes::Bytes, Self::Error> {
                let json = s3_bucket::serde_json::to_string(&self)
                    .map_err(|_| s3_bucket::error::Error::TryIntoByteError)?;
                Ok(s3_bucket::bytes::Bytes::from_owner(json))
            }
        }

        impl TryFrom<s3_bucket::bytes::Bytes> for #struct_name_expr {
            type Error = s3_bucket::error::Error;

            fn try_from(value: s3_bucket::bytes::Bytes) -> Result<Self, Self::Error> {
                let string = str::from_utf8(&value)
                    .map_err(|_|  s3_bucket::error::Error::ByteArrayToString)?;
                s3_bucket::serde_json::from_str(string)
                    .map_err(|_| s3_bucket::error::Error::TryFromByteError)
            }
        }
    }
}

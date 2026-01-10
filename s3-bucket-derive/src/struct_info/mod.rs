use regex::Regex;
use syn::{Data, DeriveInput, Field, Fields};

#[derive(Debug)]
pub struct StructInfo {
    pub struct_name: String,
    pub bucket: Option<String>,
    pub key: Option<Key>,
    pub content_type: Option<String>,
    pub fields: Vec<FieldInfo>,
}

#[derive(Debug)]
pub struct Key {
    pub value: String,
    pub arguments: Vec<String>,
}

impl Key {
    pub fn new(mut value: String) -> Self {
        value = value.replace("\"", "");
        let regex = Regex::new(r"\{([^}]*)\}").unwrap();

        let finds: Vec<_> = regex.find_iter(&value).collect();

        let mut arguments = vec![];

        for find in finds {
            arguments.push(
                find.as_str()
                    .trim_start_matches("{")
                    .trim_end_matches("}")
                    .to_string(),
            )
        }

        Self { value, arguments }
    }

    pub fn is_static_key(&self) -> bool {
        self.arguments.len() == 0
    }
}

impl StructInfo {
    pub fn new(struct_name: String) -> Self {
        Self {
            struct_name,
            bucket: None,
            key: None,
            content_type: None,
            fields: vec![],
        }
    }

    pub fn perform_checks(&self) {
        let Some(key) = &self.key else {
            return;
        };

        for argument in &key.arguments {
            if !self.field_exists(argument) {
                panic!("Field {argument} provided in the key does not exists.");
            }
        }
    }

    pub fn field_exists(&self, field_name: &String) -> bool {
        self.fields.iter().any(|x| &x.name == field_name)
    }

    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = Some(content_type.replace("\"", ""))
    }

    pub fn set_key(&mut self, key: String) {
        let key = Key::new(key);
        self.key = Some(key)
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    pub name: String,
}

impl FieldInfo {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl From<DeriveInput> for StructInfo {
    fn from(input: DeriveInput) -> Self {
        let mut struct_info = Self::new(input.ident.to_string());
        for attribute in input.attrs {
            if !attribute.path().is_ident("s3_item_prop") {
                continue;
            }

            let _ = attribute.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    return Ok(());
                };

                match ident.to_string().as_str() {
                    "bucket" => {
                        let Ok(value) = meta.value() else {
                            panic!("Error while getting pk key for struct.");
                        };
                        struct_info.bucket = Some(value.to_string());
                    }
                    "key" => {
                        let Ok(value) = meta.value() else {
                            panic!("Error while getting pk key value for struct.");
                        };
                        struct_info.set_key(value.to_string());
                    }
                    "content_type" => {
                        let Ok(value) = meta.value() else {
                            panic!("Error while getting pk key value for struct.");
                        };

                        struct_info.set_content_type(value.to_string());
                    }
                    _ => {}
                }
                return Ok(());
            });
        }

        let fields = if let Data::Struct(data) = input.data {
            if let Fields::Named(fields) = data.fields {
                fields
            } else {
                panic!("Only named fields are supported.");
            }
        } else {
            panic!("Only structs are supported.");
        };

        for field in fields.named.iter() {
            struct_info.fields.push(FieldInfo::from(field));
        }
        struct_info.perform_checks();
        return struct_info;
    }
}

impl From<&Field> for FieldInfo {
    fn from(field: &Field) -> Self {
        let field_info =
            FieldInfo::new(field.ident.as_ref().unwrap().to_string().replace("\"", ""));
        return field_info;
    }
}

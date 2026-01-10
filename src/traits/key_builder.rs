pub trait KeyBuilder {
    fn build_key(value: Vec<Box<dyn std::fmt::Display>>) -> String;
}

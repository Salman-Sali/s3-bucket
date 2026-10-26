pub trait HasContentType {
    fn get_content_type() -> String;

    /// `Cache-Control` sent with the object on put, e.g. `max-age=60`. `None` (the default)
    /// sends no `Cache-Control`. Set it with `#[s3_item_prop(cache_control = "...")]`.
    fn get_cache_control() -> Option<String> {
        None
    }
}

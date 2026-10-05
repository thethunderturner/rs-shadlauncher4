pub struct Search<'a> {
    pub query: Option<&'a str>,
    pub refresh_list: bool,
}
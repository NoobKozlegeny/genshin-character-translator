#[derive(Clone)]
pub struct Character {
    pub hu_names: Vec<String>,
    pub en_name: String
}

impl Character {
    pub fn new<T: Into<String> + Clone>(hu_names: &[T], en_name: T, ) -> Self {
        Self {
            hu_names: hu_names.to_vec().into_iter().map(Into::into).collect(),
            en_name: en_name.into()
        }
    }
}
use std::{vec::Vec, collections::HashMap};

pub trait VecExt {
    fn trim(&mut self) -> Self;
    fn replace_accents(&self) -> Vec<String>;
}

impl VecExt for Vec<&str> {
    fn trim(&mut self) -> Self {
        for i in 0..self.len() {
            self[i] = self[i].trim();
        }
        self.to_vec()
    }
    
    fn replace_accents(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let accents: HashMap<&str, &str> = HashMap::from([
            ("Á", "A"), ("É", "E"), ("Ó", "O"),
            ("Ö", "O"), ("Ő", "O"), ("Ú", "U"),
            ("Ü", "U"), ("Ű", "U"), ("Í", "I")
        ]);
        
        for name in self {
            let mut name_modified = String::new();

            for (j, char) in name.chars().enumerate() {
                if let Some(x) = accents.get(&char.to_string()[..]) {
                    name_modified += x
                } else {
                    name_modified += &char.to_string()[..];
                }
            }
            result.push(name_modified);
        }

        result
    }
}
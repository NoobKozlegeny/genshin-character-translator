use std::{vec::Vec, collections::HashMap};

pub trait VecExt {
    fn replace_accents(&self) -> Self;
}

impl VecExt for Vec<String> {    
    fn replace_accents(&self) -> Self {
        let mut result: Vec<String> = Vec::new();
        let accents: HashMap<&str, &str> = HashMap::from([
            ("Á", "A"), ("É", "E"), ("Ó", "O"),
            ("Ö", "O"), ("Ő", "O"), ("Ú", "U"),
            ("Ü", "U"), ("Ű", "U"), ("Í", "I")
        ]);
        
        for name in self {
            let mut name_modified: String = String::new();

            for (j, char) in name.chars().enumerate() {
                if let Some(x) = accents.get(&char.to_string()[..]) {
                    name_modified += x
                } else {
                    name_modified += &char.to_string()[..];
                }
            }
            result.push(name_modified);
        }

        result.clone()
    }
}

impl VecExt for Vec<char> {
    fn replace_accents(&self) -> Self {
        let mut result: Vec<char> = Vec::new();
        let accents: HashMap<char, char> = HashMap::from([
            ('Á', 'A'), ('É', 'E'), ('Ó', 'O'),
            ('Ö', 'O'), ('Ő', 'O'), ('Ú', 'U'),
            ('Ü', 'U'), ('Ű', 'U'), ('Í', 'I')
        ]);
        
        for name in self {
            if let Some(x) = accents.get(name) {
                result.push(x.clone());
            } else {
                result.push(name.clone());
            }
        }

        result
    }
}
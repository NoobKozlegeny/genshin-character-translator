use std::{vec::Vec, collections::HashMap};

use super::character::Character;

pub trait VecAccentExt {
    /// Replaces accents to english 'equivalents'
    /// # Example
    ///     Á -> A
    ///     Ó -> O
    fn replace_accents(&self) -> Self;
}

impl VecAccentExt for Vec<String> {    
    fn replace_accents(&self) -> Self {
        let mut result: Vec<String> = Vec::new();
        let accents: HashMap<&str, &str> = HashMap::from([
            ("Á", "A"), ("É", "E"), ("Ó", "O"),
            ("Ö", "O"), ("Ő", "O"), ("Ú", "U"),
            ("Ü", "U"), ("Ű", "U"), ("Í", "I")
        ]);
        
        for name in self {
            let mut name_modified: String = String::new();

            for char in name.chars() {
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

impl VecAccentExt for Vec<char> {
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

pub trait VecChararcterExt {
    fn contains_character(&self, hu_name: String) -> bool;
    fn get_en_name(&self, hu_name: String) -> Option<String>;
}

impl VecChararcterExt for Vec<Character> {
    fn contains_character(&self, hu_name: String) -> bool {
        for character in self {
            if character.hu_names.contains(&hu_name) {
                return true;
            }
        }

        return false;
    }

    fn get_en_name(&self, hu_name: String) -> Option<String> {
        for character in self {
            if character.hu_names.contains(&hu_name) {
                return Some(character.en_name);
            }
        }

        return None;
    }
}
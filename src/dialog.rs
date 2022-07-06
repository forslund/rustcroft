use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;
use glob::glob;

extern crate regex;


pub type DialogData = HashMap<String, String>;

// TODO: Cache the loaded dialogs
//let DialogCache = HashMap<String, DialogData>;

#[allow(dead_code)]
fn render(dialog: &String, data: &DialogData) -> String {
    let mut result = dialog.clone();
    for (key, value) in data {
        let replace_regex = regex::Regex::new(
                                format!(r"\{{{}\}}", key).as_str()
                            ).unwrap();
        result = replace_regex.replace_all(&result, value).into_owned();
    }
    result
}


fn read_lines<P>(filename: P) ->
        io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


struct Dialog {
    dialog_strings: Vec<String>
}

impl From<&Path> for Dialog {
    fn from(dialog_path: &Path) -> Self {
        Dialog::from_path(dialog_path)
    }
}

impl Dialog {
    #[allow(dead_code)]
    pub fn from_path(dialog_path: &Path) -> Self {
        let mut dialog = Dialog { dialog_strings: Vec::<String>::new() };
        if let Ok(lines) = read_lines(dialog_path) {
            for line in lines {
                if let Ok(dialog_line) = line {
                    dialog.dialog_strings.push(
                        String::from(dialog_line.trim())
                    );
                };
            }
        }
        dialog
    }

    #[allow(dead_code)]
    pub fn get(self: &Dialog, data: &DialogData)
            -> Result<String, &'static str> {
        if self.dialog_strings.is_empty() {
            Err("No dialogs")
        } else {
            let index = rand::thread_rng().gen_range(
                0..self.dialog_strings.len()
            );
            Ok(render(&self.dialog_strings[index], data)) //TODO: RANDOMIZE
        }
    }
}

pub struct DialogCollection {
    dialogs: HashMap<String, Dialog>
}

impl From<&str> for DialogCollection {
    fn from(folder: &str) -> Self {
        DialogCollection::from(Path::new(folder))
    }
}

impl From<&Path> for DialogCollection {
    fn from(folder: &Path) -> Self {
        let mut collection = DialogCollection {
            dialogs: HashMap::<String, Dialog>::new()
        };
        // Get all files in folder ending with .dialog
        let dialog_files = glob(folder.to_str().unwrap()).unwrap();
        for dialog_file_path in dialog_files {
            match dialog_file_path {
                Ok(path) => {
                    let p = Path::new(&path);
                    let key = String::from(
                        p.file_stem().unwrap().to_str().unwrap()
                    );
                    collection.dialogs.insert(key,
                                              Dialog::from(p));
                    },
                Err(e) => println!("{:?}", e),
            }
        }
        collection
    }
}

impl DialogCollection {
    #[allow(dead_code)]
    pub fn from_folder(path: &str, lang: &str) -> Self {
        let mut collection = DialogCollection {
            dialogs: HashMap::<String, Dialog>::new()
        };
        let dialog_folder = Path::new(path).join(lang).join("*.dialog");
        // Get all files in folder ending with .dialog
        let dialog_files = glob(dialog_folder.to_str().unwrap()).unwrap();
        for dialog_file_path in dialog_files {
            match dialog_file_path {
                Ok(path) => {
                    let p = Path::new(&path);
                    let key = String::from(
                        p.file_stem().unwrap().to_str().unwrap()
                    );
                    collection.dialogs.insert(key,
                                              Dialog::from(p));
                    },
                Err(e) => println!("{:?}", e),
            }
        }
        collection
    }

    #[allow(dead_code)]
    pub fn get(self: &DialogCollection, dialog: &str, data: &DialogData)
            -> Option<String> {
        match self.dialogs.get(dialog) {
            Some(dialog_set) => Some(dialog_set.get(data).unwrap()),
            None => None
        }
    }
}

#[macro_export]
macro_rules! dialog_data{
    ($list:expr) => {
        {
            let mut data = HashMap::new();
            for keyval in $list.iter() {
                data.insert(String::from((*keyval).0),
                            String::from((*keyval).1));
            }
            data
        }
    }
}




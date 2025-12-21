use crate::core;
use std::{
    path::{Path, PathBuf},
};
pub struct Lexer {
    json_string: Option<String>,
    file_path: Option<PathBuf>,
}

impl Lexer {
    pub fn new(json_string: Option<String>, file_path: Option<String>) -> Self {
        match (json_string, file_path) {
            (Some(str), Some(path)) => {
                panic!("both json_string and file_path are supplied choose one");
            }
            (Some(str), None) => {

            }
            (None, Some(path))=>{
                let path = Path::new(&path);
                if(!path.exists()){
                    panic!("the file_path : {} doesnt exist", &path);
                } else {
                        
                }
            }
            (None , None) => {
                panic!("neither json_string nor file_path was provided‘");
            }
        }
    }
}

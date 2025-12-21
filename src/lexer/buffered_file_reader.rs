use std::{fs::File, path::{Path, PathBuf}};

mod buffered_filereader;

pub struct BufferedFileReader {
    path: PathBuf,
    chunk_size:usize,
    file : Option<File>,
    
}

impl BufferedFileReader {
    pub fn new(path:PathBuf) -> Self {
        if(!Path::exists(&path)){
            panic!("The provided path {} doesnt exist",path.to_str().unwrap_or_default());
        } else 
        {    
            let file = File::read
        }
    }

    pub fn new_with_chunk_size(path:PathBuf, chunk_size:usize) -> Self {

    }
}
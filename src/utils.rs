use crate::structures::{Folderlink, Filelink, Pathmeta};
use std::fs::{metadata, Metadata};
use std::io::Error;

pub fn fix_pathstr(dir: String) -> String {
    dir.replace("\\\\","\\").replace("//","\\").replace("/","\\")
}

pub fn fix_pathstr_double(dir: String) -> String {
    dir.replace("\\","\\\\").replace("//","\\\\").replace("/","\\\\").replace("/","\\\\")
}

pub fn extract_filename(dir: String) -> String {
    dir.split("\\").last().unwrap().parse().unwrap()
}

pub fn sort_path(path: String) -> Pathmeta {

    let mut o: Pathmeta = Pathmeta::null;

    match metadata(path.clone()) {
        Ok(m) => {
            if m.is_dir(){
                o = Pathmeta::folder(Folderlink {
                    path: path.parse().unwrap(),
                    is_loaded: false,
                    contains: vec![]
                });
            }

            if m.is_file(){
                o = Pathmeta::file(Filelink {
                    path: path.parse().unwrap()
                });
            }
        }
        _ => {}
    }

    o

}

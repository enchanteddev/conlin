use std::fs;
use std::collections::HashMap;
use walkdir::{WalkDir, DirEntry};
// use colored::*;

fn scan_dir(dir: &str) -> HashMap<String, (u64, i32)>{
    let mut dir_data: HashMap<String, (u64, i32)> = HashMap::new();

    let mut sub_dirs: Vec<DirEntry> = vec![];
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_dir() && entry.path().to_str().unwrap() != dir {
            sub_dirs.push(entry);
            continue;
        }


        let try_reading = fs::read_to_string(entry.path());

        if try_reading.is_err(){
            continue;
        }

        let file_type_option = entry.path().extension();
        let file_type: String;
        match file_type_option {
            Some(i) => {
                file_type = i.to_str().unwrap().to_owned();
            }
            None => {
                file_type = "Other".to_string();
            }
        }
        let file_size = entry.metadata().unwrap().len();
        let file_text = try_reading.unwrap();

        let mut loc = 1;

        for ch in file_text.chars(){
            if ch == '\n'{
                loc += 1;
            }
        }

        if !dir_data.contains_key(&file_type){
            dir_data.insert(file_type, (file_size, loc));
        } else {
            let (old_fs, old_loc) = dir_data.get(&file_type).unwrap();
            dir_data.insert(file_type, (file_size + old_fs, loc + old_loc));
        }

        // println!("{}", entry.path().display());
    }

    for d in sub_dirs{
        let p = d.path().to_str().unwrap();
        let new_data = scan_dir(p);
        for key_value in new_data{
            let (file_type, (file_size, loc)) = key_value;
            if !dir_data.contains_key(&file_type){
                dir_data.insert(file_type, (file_size, loc));
            } else {
                let (old_fs, old_loc) = dir_data.get(&file_type).unwrap();
                dir_data.insert(file_type, (file_size + old_fs, loc + old_loc));
            }
        }
    }

    dir_data
}

fn main() {
    let data = scan_dir(".");
    for (key, (s, l)) in data {
        println!("{}: {}B {} Lines", key, s, l);
    }
}

use std::fs;
use std::str;
use std::env;

use rand::Rng;
use std::collections::HashMap;
use ignore::WalkBuilder;
use termsize;
use colored::*;

fn scan_dir(dir: &str) -> HashMap<String, (u64, i32)>{
    let mut dir_data: HashMap<String, (u64, i32)> = HashMap::new();
    for entry in WalkBuilder::new(dir).hidden(true).build().into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_dir() && entry.path().canonicalize().unwrap() != fs::canonicalize(dir).unwrap() {
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
    }

    dir_data
}


fn parse_data(data: HashMap<String, (u64, i32)>) -> HashMap<String, f32>{
    let mut total_lines = 0;
    let mut _total_size = 0;
    
    for (_, (s, l)) in data.clone() {
        total_lines += l;
        _total_size += s;
    }

    let mut loc_fractional_data: HashMap<String, f32> = HashMap::new();
    for (t, (_, l)) in data.clone() {
        loc_fractional_data.insert(t, l as f32 / total_lines as f32);
    }

    loc_fractional_data
}


fn get_random_color() -> String{
    let colors = vec![
        "Red",
        "Green",
        "Yellow",
        "Blue",
        "Magenta",
        "Cyan"
    ];


    colors[rand::thread_rng().gen_range(0..colors.len())].to_string()
}

fn center_string(input: &str, width: usize) -> String {
    let padding = (width.saturating_sub(input.len())) / 2;
    format!("{:width$}{}{:width$}", "", input, "", width = padding)
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let data: HashMap<String, (u64, i32)> = scan_dir(&args[1]);
    let frxnl_data = parse_data(data.clone());
    let entries = frxnl_data.len();
    let termsize::Size {cols, ..} = termsize::get().unwrap();
    
    for (ft, frxn) in frxnl_data {
        if frxn < 0.03 {continue;}
        let width_raw = (cols - entries as u16) as f32 * frxn;
        let width = width_raw.round() as u16;
        let thumbnail = format!("{}: {}", ft, data.get(&ft).unwrap().1);
        print!("{} ", (center_string(&thumbnail, width as usize)).black().on_color(get_random_color()));
    }
}

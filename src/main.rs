use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::f64::consts::PI;

struct Folder {
    map: HashMap<String, usize>,
}

impl Folder {
    fn new() -> Self{
        Folder { 
            map: HashMap::new(),
        }
    }

    fn get_all_extensions(&mut self, path: &PathBuf) {
        for entry in path.read_dir().expect("read_dir should read entry") {
            if let Ok(entry) = entry {
                let temp_path = entry.path();
                if temp_path.is_dir() {
                    self.get_all_extensions(&temp_path);
                } else {
                    let ext = match temp_path.extension() {
                        None => String::from("Undefined"),
                        Some(os_str) => String::from(os_str.to_str().unwrap())
                    };
                    let count = self.map.entry(ext).or_insert(0);
                    *count += 1;
                }
            }
        }
    }
}


fn main() {
    let destination = String::from("/home/kuba/studia/");

    let path = PathBuf::from(destination);
    let mut folder = Folder::new();

    folder.get_all_extensions(&path);

    //let indent: usize = 0; // ??? does it have to be here?
    //display_all_files(&folder.path, indent);
    // change them into more usefull methods
    print_hm(&folder.map);
    default_color_text();
    ext_to_pie_chart(&folder.map);
}


#[warn(dead_code)]
fn display_all_files(path: &Path, indent: usize) { 
    for entry in path.read_dir().expect("read_dir should read entry") {
        if let Ok(entry) = entry {
            let temp_path = entry.path();

            println!("{:ident$}{}", "", temp_path.display(), ident=indent);
            
            if entry.path().is_dir() {
                display_all_files(&temp_path, indent+1);
            }
        }
    }
}



fn print_pie_chart(k: &Vec<char>, v: &Vec<f64>, r: i32) {
    fn s(k: &Vec<char>, v: &Vec<f64>, a: f64) -> char {
        if v.is_empty() {
            return ' '
        }
        if a<v[0] {
            return k[0]
        }
        s(&(k[1..].to_vec()), &(v[1..].to_vec()), a-v[0])
    }

    let range = (-r)..r;
    for y in range.clone() {
        let mut pie = String::new();
        for x in range.clone() {
            if x*x + y*y < r*r {
                let a = (y as f64).atan2(x as f64) / PI /2.0+0.5;
                let letter = s(k, v, a);
                pie.push_str(&change_color_text(letter as u8, &letter));
            } else {
                pie.push(' ')
            }
        }
        println!("{}", pie)
    }
    default_color_text();
}

fn change_color_text(color: u8, text: &char) -> String {
    format!("\x1b[38;5;{}m{}", color, text)
}

fn default_color_text() {
    println!("\x1b[0m");
}

fn ext_to_pie_chart(map: &HashMap<String, usize>) {
    let n: usize = map.values().sum();
    
    let mut v: Vec<f64> = Vec::new();

    for (_, value) in map.iter() {
        v.push(*value as f64/n as f64 );
    }

    let mut k: Vec<char> = Vec::new();
    let base_ascii = 97;
    for i in base_ascii..base_ascii+v.len() {
        k.push( (i as u8) as char)
    }
    let r = 15;
    print_pie_chart(&k, &v, r);
}

fn print_hm(map: & HashMap<String, usize>) {
    for (key, value) in map.iter() {
        println!("{key}: {value}");
    }
}


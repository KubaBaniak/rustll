use std::fs::File;
use std::mem::size_of;
use std::path::Path;
use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Debug)]
struct FileProb {
    ext: String,
    prob: f32,
}


fn main() {
    let destination = String::from("./");
    let path = Path::new(&destination);

    let indent: usize = 0; // ??? does it have to be here?
    display_all_files(path, indent);

    let mut file_ext: HashMap<String, usize> = HashMap::new();
    get_all_extensions(path, &mut file_ext);
    print_hm(&file_ext);
    println!("{}", change_color_text(226, "siema"));
    default_color_text();
    println!("SIEEMA");

    ext_to_pie_chart(&file_ext);
}



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

fn get_all_extensions(path: & Path, map: &mut HashMap<String, usize>) {
    for entry in path.read_dir().expect("read_dir should read entry") {
        if let Ok(entry) = entry {
            let temp_path = entry.path();
            if temp_path.is_dir() {
                get_all_extensions(&temp_path.clone(), map);
            } else {
                let ext = match temp_path.extension() {
                    None => String::from("Undefined"),
                    Some(os_str) => String::from(os_str.to_str().unwrap())
                };
                let count = map.entry(ext).or_insert(0);
                *count += 1;
            }
        }
    }
}

fn print_hm(map: & HashMap<String, usize>) {
    for (key, value) in map.iter() {
        println!("{key}: {value}");
    }
}


fn print_pie_chart(k: &Vec<char>, v: &Vec<f32>, r: i32) {

    fn s(k: &Vec<char>, v: &Vec<f32>, a: f32) -> char {
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
                let a = (y as f32).atan2(x as f32) / PI /2.0+0.5;
                pie.push(s(k, v, a));
            } else {
                pie.push(' ')
            }
        }
        println!("{}", pie)
    }
}

fn change_color_text(color: usize, text: &str) -> String {
    format!("\x1b[38;5;{}m{}", color, text)
}

fn default_color_text() {
    println!("\x1b[0m");
}

fn ext_to_pie_chart(map: &HashMap<String, usize>) {
    let n: usize = map.values().sum();
    
    let mut parts: Vec<FileProb> = Vec::new();

    for (key, value) in map.iter() {
        parts.push(FileProb { ext: key.clone(), prob: *value as f32/n as f32 });
    }

    println!("{:?}\n{}", parts, parts.len());

    let mut k: Vec<char> = Vec::new();
    for i in 65..65+parts.len() {
        k.push( (i as u8) as char)
    }
    println!("{:?}", k);
    let v = parts.into_iter().map(|p| p.prob).collect();

    let r = 15;
    print_pie_chart(&k, &v, r);
}

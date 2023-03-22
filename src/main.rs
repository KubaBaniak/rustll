use std::path::Path;
use std::collections::HashMap;
use std::f32::consts::PI;


fn main() {
    let destination = String::from("./");
    let path = Path::new(&destination);

    let indent: usize = 0; // ??? does it have to be here?
    display_all_files(path, indent);

    let mut file_ext: HashMap<String, usize> = HashMap::new();
    get_all_extensions(path, &mut file_ext);
    print_hm(&file_ext);
    let k = vec!['$', '*', '@', 'x'];
    let v = vec![0.25, 0.125, 0.125, 0.125];
    let r = 8;
    pie_chart(&k, &v, r);
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

fn s(k: &Vec<char>, v: &Vec<f32>, a: f32) -> char {
    if v.is_empty() {
        return ' '
    } else if a<v[0] {
        return k[0]
    }
    return s(&(k[1..].to_vec()), &(v[1..].to_vec()), a-v[0])
}

fn pie_chart(k: &Vec<char>, v: &Vec<f32>, r: i32) {
    for y in (-r)..r {
        let mut t = String::new();
        for x in (-r)..r {
            if x*x + y*y < r*r {
                let a = (y as f32).atan2(x as f32) / PI /2.0+0.5;
                t.push(s(k, v, a));
            } else {
                t.push(' ')
            }
        }
        println!("{}", t)
    }
}

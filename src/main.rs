extern crate fs_extra;
use fs_extra::dir;
use size_format::SizeFormatterSI;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
fn walkdir(
    cur_dir: &str,
    mut collected_dirs: &mut Vec<String>,
    ftypes: &Vec<&str>,
    mut counter: &mut Vec<usize>,
    mut size_total: &mut u64,
) {
    let mut found_file = false;
    let mut links = Vec::<String>::new();
    for entry in fs::read_dir(cur_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();

        if metadata.is_file() {
            let file_name = path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .to_lowercase();
            for (i, ftype) in ftypes.iter().enumerate() {
                if file_name.ends_with(ftype) {
                    //println!("Found {:?}", file_name);
                    counter[i] += 1;
                    if !found_file {
                        collected_dirs.push(cur_dir.to_string());
                        *size_total += dir::get_size(cur_dir).unwrap();
                    }
                    found_file = true;
                }
            }
        } else if metadata.is_dir() {
            let path_name = path.to_string_lossy().to_string();
            links.push(path_name);
        }
    }
    if !found_file {
        for link in links {
            walkdir(
                &link,
                &mut collected_dirs,
                &ftypes,
                &mut counter,
                &mut size_total,
            );
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let source_dir = &args[1];

    let ftypes = vec![
        ".obj", ".fbx", ".blend", ".glb", ".gltf", ".ply", ".abc", ".stl",
    ];
    let mut size_total = 0 as u64;
    let mut counter = vec![0; ftypes.len()];
    let mut collected_dirs = Vec::<String>::new();
    walkdir(
        source_dir,
        &mut collected_dirs,
        &ftypes,
        &mut counter,
        &mut size_total,
    );
    println!("\n>>>> The Collector <<<<");

    for (i, ftype) in ftypes.iter().enumerate() {
        println!("{:7} {}", ftype, counter[i])
    }

    println!("Total size: {}B", SizeFormatterSI::new(size_total));

    if args.len() > 2 {
        let target_dir = &args[2];
        let mut options = dir::CopyOptions::new();
        options.copy_inside = true;

        for dir in collected_dirs.iter() {
            let time_stamp = "_".to_string() + &format!("{:?}", SystemTime::now())[42..51];

            let new_name = Path::new(dir)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
                + &time_stamp;
            let target_path = Path::new(&target_dir).join(new_name);
            fs::create_dir_all(&target_path).unwrap();
            fs_extra::copy_items(&vec![dir], target_path, &options).unwrap();
        }

        println!("Copied {} directories.", collected_dirs.len());
    }
}

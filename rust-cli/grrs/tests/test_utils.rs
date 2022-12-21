use std::{fs::{File}, path::Path};

pub fn get_test_file_with_contents(name: &str, contents:&str) -> &File {
    let  path = get_unused_filename();
    return File::open(path);
}

fn get_unused_filename() -> String {
    let files = std::fs::read_dir(Path::new(".")).unwrap();
    let file_suffix = "-testing.txt";
    let mut file_no = 1;
    for file in files {
        let dir = file.unwrap();
    }

    return file_no
        .to_string()
        .to_owned()
        .push_str(&file_suffix);
}
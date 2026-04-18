use std::fs;
use std::path::Path;

fn ensure_dir_exists(dir: &impl AsRef<Path>) {
    let Ok(meta) = fs::metadata(dir.as_ref()) else {
        fs::create_dir(dir.as_ref()).expect("dir.create.ok");
        return;
    };
    assert!(meta.is_dir());
}

pub fn get_next_file_id(dir: impl AsRef<Path>, prefix: &str) -> usize {
    ensure_dir_exists(&dir);

    let mut max: usize = 0;
    let mut count: usize = 0;

    for name in read_dir(&dir) {
        if name.len() <= prefix.len() {
            continue;
        }

        let suffix = &name[prefix.len()..];
        let num: usize = suffix.parse().expect("parse.ok");

        if max < num {
            max = num;
        }

        count += 1;
    }

    if count == 0 { 0 } else { max + 1 }
}

pub fn read_dir(dir: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let files = fs::read_dir(dir.as_ref()).expect("dir.read.ok");

    files.map(|f| f.unwrap().file_name().into_string().unwrap())
}

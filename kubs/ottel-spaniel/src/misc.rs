use std::fs;
use std::fs::File;
use std::io::BufWriter;
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
        if !name.starts_with(prefix) {
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
    let mut files = files
        .map(Result::unwrap)
        .map(|entry| (
            entry.metadata().unwrap().created().unwrap(),
            entry.file_name().into_string().unwrap(),
        ))
        .collect::<Vec<_>>();

    files.sort_by(|(a, _), (b, _)| a.cmp(&b));
    files.into_iter().map(|(_, f)| f)
}

pub fn open_file(path: impl AsRef<Path>) -> BufWriter<File> {
    tracing::info!(file = ?path.as_ref(), "Opening file");

    let file = File::options()
        .write(true)
        .create_new(true)
        .open(path.as_ref())
        .expect("writer.file.open");

    BufWriter::new(file)
}

pub fn load_existing_files(dir: impl AsRef<Path>, prefix: &str) -> Vec<Box<Path>> {
    let mut result = Vec::with_capacity(8);

    for name in read_dir(dir.as_ref()) {
        if !name.starts_with(prefix) {
            continue;
        }

        let mut path = dir.as_ref().to_path_buf();
        path.push(name);

        result.push(path.into_boxed_path());
    }

    result
}

use std::path::Path;
use std::fs;

pub(crate) fn get_next_file_id(
    dir: impl AsRef<Path>,
    prefix: &str
) -> usize {
    let Ok(meta) = fs::metadata(dir.as_ref()) else {
        fs::create_dir(dir.as_ref()).expect("dir.create.ok");
        return 0;
    };

    assert!(meta.is_dir());

    let files = fs::read_dir(dir.as_ref()).expect("dir.read.ok");
    let mut max: usize = 0;
    let mut count: usize = 0;

    for file in files.map(Result::unwrap) {
        let name = file.file_name().into_string().expect("osstring.convert");

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
    
    if count == 0 {
        0
    } else {
        max + 1
    }
}

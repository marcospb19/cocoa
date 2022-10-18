use std::{
    env,
    io::{self, stdin, Read},
    path::PathBuf,
};

use fs_err as fs;

pub fn read_input() -> io::Result<String> {
    let files: Vec<PathBuf> = env::args().skip(1).map(PathBuf::from).collect();

    if files.is_empty() {
        let mut buf = String::with_capacity(1024);
        stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else if files.len() == 1 {
        fs::read_to_string(&files[0])
    } else {
        panic!("CLI ERROR: expected 0 or 1 files, found {}", files.len());
    }
}

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

const BUFFER_SIZE: usize = 8 * 1024 * 1024; // 8 MB

fn get_files_in_directory(path: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?.path();
        if entry.is_file() {
            files.push(entry);
        } else {
            files.extend(get_files_in_directory(entry)?);
        }
    }
    Ok(files)
}

fn get_files_to_combine(paths: Vec<PathBuf>) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];
    for path in paths {
        if path.is_file() {
            files.push(path)
        } else {
            files.extend(get_files_in_directory(path)?)
        }
    }
    Ok(files)
}

fn combine_files(files: &[PathBuf], output_file: &mut BufWriter<File>) -> io::Result<()> {
    for file in files {
        let mut input_file = BufReader::with_capacity(BUFFER_SIZE, File::open(file)?);
        io::copy(&mut input_file, output_file)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let paths = env::args().skip(1).map(PathBuf::from).collect();
    let files_to_combine = get_files_to_combine(paths)?;
    println!("combining {} files", files_to_combine.len());

    let output_file_path = Path::new("combined.txt");
    let output_file = BufWriter::with_capacity(BUFFER_SIZE, OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file_path)?);

    let output_file = Arc::new(Mutex::new(output_file));

    let chunk_size = (files_to_combine.len() as f32 / num_cpus::get() as f32 * 2.0).ceil() as usize;
    files_to_combine.par_chunks(chunk_size).try_for_each(|chunk| {
        let output_file = output_file.clone();
        let mut output_file = output_file.lock().unwrap();
        combine_files(chunk, &mut output_file)
    })
}

use core::str;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

/// Backup files from source to target directory
pub fn backup(source_dir: &str, target_dir: &str) -> bool {
    let (source_path, target_path) = assert_source_and_target_dirs_exist(source_dir, target_dir);
    return copy_files(source_path, target_path, false).len() > 0;
}

/// Backup files from source to target directory and convert them to JPEG
pub fn backup_and_convert(source_dir: &str, target_dir: &str) -> bool {
    let (source_path, target_path) = assert_source_and_target_dirs_exist(source_dir, target_dir);
    copy_files(source_path, target_path, true)
        .iter()
        .map(|file| {
            convert_to_jpeg(file);
        })
        .count()
        > 0
}

fn assert_source_and_target_dirs_exist<'a>(
    source_dir: &'a str,
    target_dir: &'a str,
) -> (&'a Path, &'a Path) {
    let source_path = Path::new(source_dir);
    let _source_path_exists = match source_path.exists() {
        true => true,
        false => panic!("Source directory does not exist"),
    };
    let _source_path_is_dir = match source_path.is_dir() {
        true => true,
        false => panic!("Source path is not a directory"),
    };
    let target_path = Path::new(target_dir);
    let _target_path_exists = match target_path.exists() {
        true => true,
        false => panic!("Target directory does not exist"),
    };
    let _target_path_is_dir = match target_path.is_dir() {
        true => true,
        false => panic!("Target path is not a directory"),
    };
    return (source_path, target_path);
}

const PHOTO_PREFIX: &str = "PRD";

/// Copy files from source to target directory
///
/// If `move_files` is true, copied files are removed from source directory. Otherwise, they are left in place.
fn copy_files(source_path: &Path, _target_path: &Path, move_files: bool) -> Vec<String> {
    let mut nr_files_to_copy: i64 = 0;
    let mut new_files = Vec::new();
    let mut files_copied = Vec::new();
    for entry in WalkDir::new(source_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map_or(false, |s| s.starts_with(PHOTO_PREFIX))
        })
    {
        nr_files_to_copy += 1;
        fs::copy(entry.path(), _target_path.join(entry.file_name()))
            .expect(format!("Failed to copy file '{}'", entry.path().display()).as_str());
        if _target_path.join(entry.file_name()).exists() {
            let copied_file = _target_path.join(entry.file_name()).display().to_string();
            new_files.push(copied_file);
            files_copied.push(entry.path().display().to_string());
        }
    }
    assert!(
        new_files.len() == nr_files_to_copy as usize,
        "Not all files were copied"
    );
    if move_files {
        for file in files_copied.iter() {
            fs::remove_file(file).expect("Failed to remove file");
        }
    }
    return new_files;
}

struct Metadata {
    year: String,
    month: String,
    day: String,
}

/// Convert file to JPEG
pub fn convert_to_jpeg(file_path: &str) {
    println!("Converting file to JPEG: {}", file_path);
    let last_known_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let meta = read_metadata(file_path);
    let new_filename = format!(
        "{}{}{}_{}.jpeg",
        meta.year, meta.month, meta.day, last_known_name
    );
    println!("New filename: {}", new_filename);
    write_jpeg(file_path, new_filename.as_str());
}

fn read_metadata(file_path: &str) -> Metadata {
    let mut buffer = [0; 54];
    let metadata = fs::File::open(file_path)
        .unwrap()
        .read(&mut buffer[..])
        .unwrap();
    let as_read = str::from_utf8(&buffer[..metadata])
        .unwrap()
        .replace("\0", "");

    let date_split: Vec<&str> = as_read
        .split("\u{4} PHOTO - ")
        .into_iter()
        .next()
        .to_owned()
        .unwrap()
        .split("/")
        .collect();
    let month_split = date_split[0].split(" ").collect::<Vec<&str>>();
    let month = month_split.last().unwrap();
    let day = date_split[1];
    let year_split = date_split[2].split(" ").collect::<Vec<&str>>();
    let year = year_split.first().unwrap();
    return Metadata {
        year: year.to_string(),
        month: month.to_string(),
        day: day.to_string(),
    };
}

/// Reads the RDR2 photo file
fn write_jpeg(file_path: &str, new_filename: &str) {
    let mut file = fs::File::open(file_path).expect("Failed to open file");
    file.seek(std::io::SeekFrom::Start(300))
        .expect("Failed to seek to 300th byte");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    // Save file with the new filename in the same directory
    let new_path = Path::new(file_path).with_file_name(new_filename);
    let mut new_file = fs::File::create(new_path).expect("Failed to create new file");
    new_file
        .write_all(&buffer)
        .expect("Failed to write to new file");
}

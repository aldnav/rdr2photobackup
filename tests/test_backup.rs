use rdr2photobackup::*;
use serial_test::serial;
use std::path::Path;

/// Resolve test file paths under `tests/rdr2/`
macro_rules! test_case {
    // ref: https://stackoverflow.com/a/74550371/3792514
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/rdr2/", $fname) // assumes Linux ('/')!
    };
}

/// Resolve test file paths under `tests/res/`
macro_rules! res {
    // ref: https://stackoverflow.com/a/74550371/3792514
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/res/", $fname) // assumes Linux ('/')!
    };
}

fn setup() {
    std::fs::copy(res!("PRDR34088256_1"), test_case!("photos/PRDR34088256_1"))
        .expect("Failed to copy test file");
    let target_dir = test_case!("backup-folder");
    std::fs::remove_dir_all(target_dir)
        .and_then(|_| std::fs::create_dir(target_dir))
        .expect("Failed to create backup folder");
}

fn tear_down() {
    let target_dir = test_case!("backup-folder");
    std::fs::remove_dir_all(target_dir).expect("Failed to remove backup folder");
    std::fs::create_dir(target_dir).expect("Failed to create backup folder");
}

#[test]
#[serial]
fn source_dir_exists() {
    setup();
    let source_dir = test_case!("photos");
    let target_dir = test_case!("backup-folder");
    let result = backup(source_dir, target_dir);
    assert_eq!(result.unwrap(), true);

    // [!] Uses panic::catch_unwind()
    // use std::panic;
    // let source_dir = "/Users/aldnav/Documents/rdr2/dne-photos";
    // let result = panic::catch_unwind(|| {
    //     backup(source_dir, target_dir);
    // });
    // assert!(result.is_err());
}

// [!] Uses should_panic
#[test]
#[serial]
#[should_panic(expected = "Source directory does not exist")]
fn source_dir_does_not_exist() {
    setup();
    let source_dir = test_case!("dne-photos");
    let target_dir = test_case!("backup-folder");
    backup(source_dir, target_dir);
}

#[test]
#[serial]
fn target_dir_exists() {
    setup();
    let source_dir = test_case!("photos");
    let target_dir = test_case!("backup-folder");
    let result = backup(source_dir, target_dir);
    assert_eq!(result.unwrap(), true);
}

#[test]
#[serial]
#[should_panic(expected = "Target directory does not exist")]
fn target_dir_does_not_exist() {
    setup();
    // [!] Uses unwrap_err()
    let source_dir = test_case!("photos");
    let target_dir = test_case!("dne-backup-folder");
    let res = backup(source_dir, target_dir);
    assert_eq!(res.unwrap(), true);
}

#[test]
#[serial]
fn file_is_copied() {
    setup();
    let source_dir = test_case!("photos");
    let target_dir = test_case!("backup-folder");
    let result = backup(source_dir, target_dir);
    assert_eq!(result.unwrap(), true);
    assert!(Path::new(test_case!("backup-folder/PRDR34088256_1")).exists());
    tear_down();
}

#[test]
#[serial]
fn file_is_copied_and_converted() {
    setup();
    let source_dir = test_case!("photos");
    let target_dir = test_case!("backup-folder");
    let result = backup_and_convert(source_dir, target_dir);
    assert_eq!(result.unwrap(), true);
    // assert!(Path::new(test_case!("backup-folder/PRDR34088256_1.jpg")).exists());
    assert!(!Path::new(test_case!("photos/PRDR34088256_1")).exists());
    tear_down();
}

#[test]
#[serial]
fn converts_to_jpeg() {
    let source_file = res!("PRDR34088256_1");
    convert_to_jpeg(source_file);
    let res_dir = Path::new(source_file).parent().unwrap();
    let mut exported_count = 0;
    let exported: Vec<_> = res_dir
        .read_dir()
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.to_str().unwrap().ends_with("PRDR34088256_1.jpeg") {
                exported_count += 1;
                Some(entry)
            } else {
                None
            }
        })
        .collect();
    assert!(exported_count >= 1);
    for entry in exported {
        let path = entry.path();
        std::fs::remove_file(path).expect("Failed to remove file");
    }
}

#[test]
#[serial]
fn backup_and_convert_files() {
    setup();
    let source_dir = test_case!("photos");
    let target_dir = test_case!("backup-folder");
    let result = backup_and_convert(source_dir, target_dir);
    assert_eq!(result.unwrap(), true);
    assert!(!Path::new(test_case!("photos/PRDR34088256_1")).exists());
    assert!(Path::new(test_case!("backup-folder/240929_PRDR34088256_1.jpeg")).exists());
    tear_down();
}

// #[test]
// #[serial]
// fn verify_source_dir_has_files() {
//     let source_dir = test_case!("photos");
//     let result = verify_has_source_files(source_dir);
//     assert_eq!(result, Ok(()));
// }

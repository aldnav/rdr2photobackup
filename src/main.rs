use rdr2photobackup::backup_and_convert;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: rdr2photobackup <source_dir> <target_dir>");
        std::process::exit(1);
    }

    let source_dir = &args[1];
    let target_dir = &args[2];

    sep();
    println!("RDR2 Photo Backup");
    println!("Source: {}", source_dir);
    println!("Target: {}", target_dir);
    sep();
    backup_and_convert(source_dir, target_dir);
}

fn sep() {
    let token: &str = "=";
    for _ in 0..80 {
        print!("{}", token);
    }
    println!();
}

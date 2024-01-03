/*
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}


fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=1_023 => FileSize::Bytes(size),
        1_024..=1_048_575 => FileSize::Kilobytes(size / 1_024),
        1_048_576..=1_073_741_823 => FileSize::Megabytes(size / 1_048_576),
        _ => FileSize::Gigabytes(size / 1_073_741_824),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", size),
        FileSize::Kilobytes(kb) => format!("{} KB", kb as f64/1024.0),
        FileSize::Megabytes(mb) => format!("{} MB", mb as f64/1024.0),
        FileSize::Gigabytes(gb) => format!("{} GB", gb as f64/1024.0),
    }
} 

This implementation is normal.
Make it better:
*/

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{} KB", *kb as f64/1024.0),
            FileSize::Megabytes(mb) => format!("{} MB", *mb as f64/1024.0),
            FileSize::Gigabytes(gb) => format!("{} GB", *gb as f64/1024.0),
        }
    }
}


fn main() {
    let size = 2_000_000_000;
    let filesize = match size {
        0..=1_023 => FileSize::Bytes(size),
        1_024..=1_048_575 => FileSize::Kilobytes(size / 1_024),
        1_048_576..=1_073_741_823 => FileSize::Megabytes(size / 1_048_576),
        _ => FileSize::Gigabytes(size / 1_073_741_824),
    };
    println!("File size: {}", filesize.format_size());
}
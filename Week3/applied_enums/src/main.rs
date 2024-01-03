use num_traits::pow;
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
    Terabytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{} bytes", *kb as f64 * 1024.0),
            FileSize::Megabytes(mb) => format!("{} KB", *mb as f64 * 1024.0),
            FileSize::Gigabytes(gb) => format!("{} MB", *gb as f64 * 1024.0),
            FileSize::Terabytes(tb) => format!("{} GB", *tb as f64 * 1024.0),
        }
    }
}

fn format_file_size(size: u64) -> String {
    match size {
        0..=1023 => format!("{} bytes", size),
        1024..=1_048_575 => format!("{:.2} KB", size as f64 / 1024.0),
        1_048_576..=1_073_741_823 => format!("{:.2} MB", size as f64 / 1024.0 / 1024.0),
        1_073_741_824..=1_099_511_627_775 => format!("{:.2} GB", size as f64 / 1024.0 / 1024.0 / 1024.0),
        _ => format!("{:.2} TB", size as f64 / 1024.0 / 1024.0 / 1024.0 / 1024.0),
    }
}

fn main() {
    let size = 1_099_511_627_777; // Represents the size of a file in bytes
    let filesize = match size {
        0..=1_023 => FileSize::Bytes(size),
        1_024..=1_048_575 => FileSize::Kilobytes(size / 1_024),
        1_048_576..=1_073_741_823 => FileSize::Megabytes(size / (1_024 * 1_024)),
        1_073_741_824..=1_099_511_627_775 => FileSize::Gigabytes(size / (1_024 * 1_024 * 1_024)),
        1_099_511_627_776..=1_125_899_906_842_623 => FileSize::Terabytes(size / (1_024 * 1_024 * 1_024 * 1_024)),
        _ => FileSize::Terabytes(size / pow(1_024, 4)),
    };
    println!("File size: {}", filesize.format_size());

    let sizes = [2500, 1_500_000, 3_000_000_000, 5_000_000_000_000];
    for size in sizes {
        println!("File size: {}", format_file_size(size));
    }
}
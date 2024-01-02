enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}


fn main() {
    let disk_type = DiskType::SSD;
    let disk_size = DiskSize::GB(128);

    match disk_type {
        DiskType::SSD => println!("Disk type is SSD"),
        DiskType::HDD => println!("Disk type is HDD"),
    }

    //println!("Disk type: {:?}", disk_type);
    println!("Disk size: {:?}", disk_size);
}
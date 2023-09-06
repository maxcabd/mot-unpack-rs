use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::{Seek, Read, Write};
use std::path::{Path, PathBuf};
use binrw::BinReaderExt;


mod structure;

use crate::structure::mot::Mot;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = Path::new(&args[1]);

    assert_eq!(filepath.extension().unwrap(), "mot");
    
    let filename =  Path::new(filepath).file_stem().unwrap().to_str().unwrap();

    let mut file = File::open(filepath).unwrap();
    let mot = file.read_le::<Mot>().unwrap();


    let mut offsets = Vec::new();
    let mut indices_hash = HashMap::new();

    offsets.push(mot.mot_header.filesize); // First offset is the end of the file (filesize)

    for k in 0..mot.mot_offset_table.offsets.len() {
        let offset = mot.mot_offset_table.offsets[k];
        if offset != 0 {
            offsets.push(mot.mot_offset_table.offsets[k]);
            indices_hash.insert(offset, k);
        }
    }
    // Reorder the offsets so that they are in descending order
    offsets.reverse();

    // Write the MOTA(s) to a file
    let mut mota_path = PathBuf::from(filepath);
    mota_path.pop(); // remove the file name
    mota_path.push(filename);
    fs::create_dir_all(&mota_path).unwrap();

    for i in 0..offsets.len() - 1 {
        file.seek(std::io::SeekFrom::Start(offsets[i] as u64)).unwrap();
        let size = offsets[i + 1] as usize - offsets[i] as usize;
        let mut buffer = vec![0; size];
        file.read_exact(&mut buffer).unwrap();

        let index_str = format!("0x{:0>4X}", indices_hash.get(&offsets[i]).unwrap_or(&0));

        let mut file = File::create(format!("{}\\{}.mota", mota_path.display().to_string(), index_str)).unwrap();
        file.write_all(&buffer).unwrap();
    }

}

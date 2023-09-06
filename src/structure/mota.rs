// Eighting Motion Animation file structure
use binrw::{io::SeekFrom, binrw};



#[binrw]
#[br(big)]
#[derive(Debug)]
pub struct Mota {
    pub section_count: i32,
    pub section_header_length: i32,
    pub play_speed: f32,
    pub end_time: f32,

    #[br(count = 1)]
    pub bones: Vec<Bone>
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub struct Bone {
    pub flag1: u8,
    pub flag2: u8,
    pub flag3: u16,
    pub bone_index: i16,
    pub key_count: u32,
    pub frame_count: f32,

    pub time_offset: u32,
    pub key_offset: u32,

    #[br(seek_before = SeekFrom::Start(time_offset as u64))]
    pub time: f32,

    #[br(if(key_offset != 0))]
    #[br(seek_before = SeekFrom::Start(key_offset as u64))]
    #[br(count = key_count)]
    pub keys: Vec<Key>
}

#[binrw]
#[br(big)]
#[derive(Debug)]
pub struct Key {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/*impl Bone {
    pub fn read_keys<R: Read + Seek>(&self, reader: &mut R) -> binrw::BinResult<Vec<Key>> {
        let start = reader.seek(SeekFrom::Current(0))?;

        let mut keys = Vec::with_capacity(self.key_count as usize);

        for k in 0..self.key_count {
            if self.key_offset != start as u32 {
                reader.seek(SeekFrom::Current(self.time_offset as i64 + 8 * k as i64));
                reader.read_le::<Key>().unwrap();
            }

            keys.push(Key { time, x, y, z, w });
        }

        Ok(keys)
}

}*/

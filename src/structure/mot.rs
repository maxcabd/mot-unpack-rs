use binrw::{binrw};

#[binrw]
#[brw(big)]
#[derive(Debug)]
pub struct Mot {
    pub mot_header: MotHeader,

    #[br(args(mot_header.mota_count))]
    pub mot_offset_table: MotOffsetTable,
    //mota: Vec<Mota>
}


#[binrw]
#[derive(Debug, Clone, Default)]
pub struct MotHeader {
    pub magic: u32,

    pub mota_count: u32,
    pub offset_table_offset: u32,
    pub filesize: u32,
}

#[binrw]
#[derive(Debug)]
#[br(import(offset_table_count: u32))]
pub struct MotOffsetTable {
    #[br(count = offset_table_count)]
    pub offsets: Vec<u32>
}
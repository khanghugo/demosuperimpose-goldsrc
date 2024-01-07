use bsp_file::bsp::{LumpType, RawMap};
use bsp_render::level::entities::parse_entities;

pub struct BspInfo {
    skyname: String,
}

pub fn get_map_info(filename: &str) -> BspInfo {
    let bsp_file = std::fs::read(filename).unwrap();
    let raw_map = RawMap::parse(&bsp_file).unwrap();

    let entities = parse_entities(raw_map.lump_data(LumpType::Entities)).unwrap();
    println!("{:?}", entities);

    BspInfo {
        skyname: "sneed".to_string(),
    }
}

use std::ops::Index;

use point::Point;

#[derive(Debug)]
pub struct QRData {
    pub data: Vec<u8>,
    pub version: u32,
    pub side: u32,
}

impl QRData {
    pub fn new(data: Vec<u8>, version: u32) -> QRData {
        QRData {
            data,
            version,
            side: 4 * version + 17,
        }
    }
}

impl Index<[u32; 2]> for QRData {
    type Output = u8;

    fn index(&self, index: [u32; 2]) -> &u8 {
        &self.data[index[1] as usize * self.side as usize + index[0] as usize]
    }
}

#[derive(Debug)]
pub struct QRLocation {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub module_size: f64,
    pub version: u32,
}

#[derive(Debug)]
pub struct QRFinderPosition {
    pub location: Point,
    pub module_size: f64,
}
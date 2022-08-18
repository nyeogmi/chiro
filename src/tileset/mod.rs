use std::convert::TryInto;

use crate::shared::*;

pub(crate) struct TileSet<'a> {
    pub buf: &'a [u8],
    pub overall_size: PixelSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile(pub [u8; 8]);

impl<'a> TileSet<'a> {
    pub fn tile(&self, ix: u32) -> Tile {
        let zels_sz = self.overall_size.to_zels();
        let n_tiles = zels_sz.width * zels_sz.height;

        if ix >= n_tiles { return Tile([0; 8]) }

        let value: [u8; 8] = self.buf[(ix * ZEL_PIXELS_Y) as usize..((ix + 1) * ZEL_PIXELS_Y) as usize].try_into().unwrap();
        Tile(value)
    }
}

impl Tile {
    pub fn blank() -> Self { Self([0; 8]) }

    pub(crate) fn left(&self) -> Tile {
        fn fix(row: u8) -> u8 { row << 4 }
        Tile([
            fix(self.0[0]), fix(self.0[1]), fix(self.0[2]), fix(self.0[3]),
            fix(self.0[4]), fix(self.0[5]), fix(self.0[6]), fix(self.0[7]),
        ])
    }

    pub(crate) fn right(&self) -> Tile {
        fn fix(row: u8) -> u8 { row >> 4 }
        Tile([
            fix(self.0[0]), fix(self.0[1]), fix(self.0[2]), fix(self.0[3]),
            fix(self.0[4]), fix(self.0[5]), fix(self.0[6]), fix(self.0[7]),
        ])
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::blank()
    }
}
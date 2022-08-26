use euclid::*;

use crate::{shared::*, tileset::{TileSet, Tile}};

mod cp437;

#[derive(Clone, Copy)]
pub enum Font {
    Normal,
    Small,
    Set, 
    Fat,
}


const BITMAP: &'static [u8; 0x1000] = include_bytes!("font.bin");
const BITMAP_SMALL: &'static [u8; 0x800] = include_bytes!("font_small.bin");
const BITMAP_FAT: &'static [u8; 0x2000] = include_bytes!("font_fat.bin");

const FONT: TileSet<'static> = TileSet {
    buf: BITMAP,
    overall_size: PixelSize::new(256, 128),
};

const FONT_SMALL: TileSet<'static> = TileSet {
    buf: BITMAP_SMALL,
    overall_size: PixelSize::new(128, 128),
};

const FONT_FAT: TileSet<'static> = TileSet {
    buf: BITMAP_FAT,
    overall_size: PixelSize::new(256, 256),
};

impl Font {
    pub(crate) fn char_size(&self) -> ZelSize {
        match self {
            Font::Normal => size2(1, 2),
            Font::Small => size2(1, 1),
            Font::Set => size2(2, 2),
            Font::Fat => size2(2, 2),
        }
    }

    pub(crate) fn char_to_tile(&self, c: char, mut cb: impl FnMut(ZelPoint, Tile)) {
        let ix = cp437::encode_lossy(c) as u32;

        match self {
            Font::Normal => {
                cb(point2(0, 0), FONT.tile(ix * 2));
                cb(point2(0, 1), FONT.tile(ix * 2 + 1));
            }
            Font::Small => {
                cb(point2(0, 0), FONT_SMALL.tile(ix));
            }
            Font::Set => {
                cb(point2(0, 0), FONT.tile(ix * 2).left());
                cb(point2(1, 0), FONT.tile(ix * 2).right());
                cb(point2(0, 1), FONT.tile(ix * 2 + 1).left());
                cb(point2(1, 1), FONT.tile(ix * 2 + 1).right());
            }
            Font::Fat => {
                cb(point2(0, 0), FONT_FAT.tile(ix * 4));
                cb(point2(1, 0), FONT_FAT.tile(ix * 4 + 1));
                cb(point2(0, 1), FONT_FAT.tile(ix * 4 + 2));
                cb(point2(1, 1), FONT_FAT.tile(ix * 4 + 3));
            }
        }
    }
}
use std::rc::Rc;

use euclid::*;

use crate::{screen::Screen, shared::*};

use super::ZelData;

pub struct PixelFB {
    buffer: Vec<u32>,

    old: Option<Screen>,
    old_selection: Option<Affordance>,
}

impl PixelFB {
    pub fn new() -> PixelFB {
        PixelFB { buffer: vec![], old: None, old_selection: None}
    }

    // true if work was done
    pub(crate) fn draw(&mut self, new: &Screen, new_selection: Option<Affordance>, dirty_cells: Option<&[(u32, u32)]>) -> bool {
        let new_px_size = new.size.to_pixels();
        let new_buf_sz = (new_px_size.width * new_px_size.height) as usize;

        let result = if let Some(old) = self.old.as_ref() {
            // TODO: fucking YUCK, this logic is ugly
            if self.buffer.len() == new_buf_sz {
                if new.size == old.size {
                    Self::draw_differences(&old, self.old_selection, &new, new_selection, &mut self.buffer, dirty_cells)
                }
                else {
                    self.completely_redraw(&new);
                    true
                }

            } else {
                self.buffer.resize(new_buf_sz, 0);
                self.completely_redraw(&new);
                true
            }

        } else {
            self.buffer.resize(new_buf_sz, 0);
            self.completely_redraw(&new);
            true
        };

        if let Some(scr) = self.old.as_mut() {
            scr.clone_from(new);
        } else {
            self.old = Some(new.clone())
        }
        self.old_selection = new_selection;

        result
    }

    fn draw_differences(old: &Screen, old_selection: Option<Affordance>, new: &Screen, new_selection: Option<Affordance>, buffer: &mut Vec<u32>, dirty_cells: Option<&[(u32, u32)]>) -> bool {
        let size = new.size;
        let w = size.width;
        let h = size.height;

        let old_wbg = old.bg;
        let old_wfg = old.fg;

        let new_wbg = new.bg;
        let new_wfg = new.fg;

        let mut touched = false;

        let mut cb = |x: u32, y: u32| {
            let (old_zd, old_id) = old.raw_view(point2(x as i32, y as i32));
            let old_zd = old_zd.adapted_for(old_wbg, old_wfg, old_selection);
            let (new_zd, new_id) = new.raw_view(point2(x as i32, y as i32));
            let new_zd = new_zd.adapted_for(new_wbg, new_wfg, new_selection);

            if !visually_identical((old_zd, old_id), (new_zd, new_id)) {
                if let Some(id) = new_id { 
                    id.physically_draw(buffer, x, y, w); 
                }
                else {
                    new_zd.physically_draw(buffer, x, y, w);
                }
                touched = true
            }
        };

        if let Some(dc) = dirty_cells {
            for c in dc { cb(c.0, c.1) }
        } else {
            for y in 0..h {
                for x in 0..w {
                    cb(x, y)
                }
            }
        }

        touched
    }

    fn completely_redraw(&mut self, new: &Screen) {
        let size = new.size;
        let w = size.width;
        let h = size.height;

        let new_wbg = new.bg;
        let new_wfg = new.fg;
        let new_mouseover = None;

        for y in 0..h {
            for x in 0..w {
                let (new_zd, new_id) = new.raw_view(point2(x as i32, y as i32));
                let new_zd = new_zd.adapted_for(new_wbg, new_wfg, new_mouseover);

                if let Some(id) = new_id { 
                    id.physically_draw(&mut self.buffer, x, y, w); 
                }
                else {
                    new_zd.physically_draw(&mut self.buffer, x, y, w);
                }
            }
        }
    }

    pub(crate) fn view_buffer(&self) -> (&[u32], PixelSize) {
        if let Some(old) = &self.old {
            return (&self.buffer, old.size.to_pixels())
        } else {
            return (&[], size2(0, 0))
        }
    }

}


// for adapted zels only
// (doesn't consider transparent colors)
fn visually_identical((old_zd, old_id): (ZelData, Option<&SuperTile>), (new_zd, new_id): (ZelData, Option<&SuperTile>)) -> bool {
    match (new_id, old_id) {
        (Some(_), None) => return false,
        (None, Some(_)) => return false,
        (Some(x), Some(y)) => return Rc::ptr_eq(&x.0, &y.0),
        (None, None) => {
            old_zd.tile == new_zd.tile && old_zd.bg == new_zd.bg && old_zd.fg == new_zd.fg
        }
    }
}
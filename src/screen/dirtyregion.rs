use crate::shared::*;

// defaults to 4800, the threshold for 80 x 60
pub struct DirtyRegion<const MAX_DIRTY: usize = 4800> {
    n_dirty: usize,
    dirty_cells: [(u32, u32); MAX_DIRTY],
}

impl<const MAX_DIRTY: usize> DirtyRegion<MAX_DIRTY> {
    pub fn new() -> Self {
        DirtyRegion {
            n_dirty: 0,
            dirty_cells: [(0, 0); MAX_DIRTY]
        }
    }

    pub fn is_dirty(&self) -> bool {
        return self.n_dirty > 0
    }

    pub fn is_saturated(&self) -> bool {
        // == MAX_DIRTY just means "we're full!"
        return self.n_dirty > MAX_DIRTY;
    }

    pub fn record(&mut self, zp: Zel) {
        if zp.x < 0 || zp.y < 0 { return; }  // screens don't go negative, so this is definitely not in bounds

        let point = (zp.x as u32, zp.y as u32);
         
        if self.n_dirty > MAX_DIRTY { return }
        if self.n_dirty > 0 && self.dirty_cells[self.n_dirty - 1] == point { return; }  // it's fairly common for this to be hit more than once
        if self.n_dirty == MAX_DIRTY { self.n_dirty += 1; return; }

        self.dirty_cells[self.n_dirty] = point;
        self.n_dirty += 1;
    }

    pub fn dirty_cells(&mut self) -> Option<&[(u32, u32)]> {
        if self.is_saturated() { return None }
        Some(&self.dirty_cells[..self.n_dirty])
    }

    pub fn reset(&mut self) {
        self.n_dirty = 0;
    }

    pub fn saturate(&mut self) {
        self.n_dirty = MAX_DIRTY + 1;
    }
}
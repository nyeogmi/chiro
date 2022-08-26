use crate::shared::*;

use super::ZelData;

impl ZelData {
    pub(crate) fn adapted_for(
        mut self,
        window_bg: Color, 
        window_fg: Color,
        click_selection: Option<Affordance>
    ) -> ZelData {
        self.fg = if self.fg.is_opaque() { self.fg } else { window_fg };
        self.bg = if self.bg.is_opaque() { self.bg } else { window_bg };

        if click_selection.is_some() && click_selection == self.click {
            (self.fg, self.bg) = (self.bg, self.fg);
        }

        self
    }

    pub(super) fn physically_draw(
        &self, 
        out_buf: &mut Vec<u32>, 
        out_x: u32, out_y: u32, out_width: u32, 
    ) {
        let real_out_x = out_x * ZEL_PIXELS_X;
        let real_out_y = out_y * ZEL_PIXELS_Y;
        let real_out_width = out_width * ZEL_PIXELS_X;

        for y in [0, 1, 2, 3, 4, 5, 6, 7] {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                let color = if self.tile.0[y as usize] >> x & 1 == 1 { self.fg } else { self.bg };
                unsafe {
                    *out_buf.get_unchecked_mut(((real_out_y + y) * real_out_width + real_out_x + x) as usize) = color.0;
                }
            }
        }
    }
}
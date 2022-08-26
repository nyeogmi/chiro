use crate::shared::*;
use euclid::*;

pub(crate) fn build_rect(xy1: ZelPointI, xy2: ZelPointI) -> ZelRectI {
    let min_x = xy1.x.min(xy2.x);
    let max_x = xy1.x.max(xy2.x);

    let min_y = xy1.y.min(xy2.y);
    let max_y = xy1.y.max(xy2.y);

    rect(min_x, min_y, max_x - min_x, max_y - min_y)
}
use std::rc::Rc;

#[derive(Clone)]
pub struct SuperTile(pub Rc<[[u32; 8]; 8]>);

impl SuperTile {
    pub fn new(data: [[u32; 8]; 8]) -> SuperTile {
        SuperTile(Rc::new(data))
    }
}
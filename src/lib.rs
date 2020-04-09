use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Row {
    e: [f32; 3],
    f: [f32; 3],
}

impl Index<usize, > for Row {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f32 {
        &mut self.e[i]
    }
}


fn main() {
    // todo
}
use crate::Array;

pub struct Table<T> {

    array: Array<T>,

    width: usize
}

impl<T> Table<T> {

    pub fn new(array: Array<T>, width: usize) -> Table<T> {
        Table {
            array: array,
            width: width
        }
    }

    pub fn index_for(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn set(&self, x: usize, y: usize, value: T){
        self.array.set(self.index_for(x, y), value);
    }

    pub fn get_ref(&self, x: usize, y: usize) -> &T {
        self.array.get_ref(self.index_for(x, y))
    }

    pub fn get_mut_ref(&self, x: usize, y: usize) -> &mut T {
        self.array.get_mut_ref(self.index_for(x, y))
    }
}

impl<T: Copy> Table<T> {

    pub fn get(&self, x: usize, y: usize) -> T {
        self.array.get(self.index_for(x, y))
    }
}
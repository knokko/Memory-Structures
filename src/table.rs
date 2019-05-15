use crate::Array;
use crate::Graphics2D;

use std::ops::{Add,AddAssign};

pub struct Table<T> {

    array: Array<T>,

    width: usize,
    height: usize,
    bound: usize
}

impl<T> Table<T> {

    pub fn new(array: Array<T>, width: usize, height: usize) -> Table<T> {
        if width == 0 || height == 0 {
            panic!("The width is {} and the height is {}, but neither can be 0", width, height);
        }
        let bound = width.checked_mul(height).unwrap();

        // This test ensures that any operation within the table bounds will also be within the Array bounds.
        array.check_bound(bound - 1);
        Table {
            array: array,
            width: width,
            height: height,
            bound: bound
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn unchecked_index_for(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    /// Gets the array index for the given x and y. This will panic if x or y
    /// is outside this table.
    pub fn index_for(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            panic!("x is {}, width is {}, y is {} and height is {}", x, self.width, y, self.height);
        }

        // If x and y are within range, which has been checked in the new method,
        // the resulting index must be safe
        self.unchecked_index_for(x, y)
    }

    pub fn set(&self, x: usize, y: usize, value: T){
        self.array.set_unchecked(self.index_for(x, y), value);
    }

    pub fn set_unchecked(&self, x: usize, y: usize, value: T){
        self.array.set_unchecked(self.unchecked_index_for(x, y), value);
    }

    pub fn get_ref(&self, x: usize, y: usize) -> &T {
        self.array.get_unchecked_ref(self.index_for(x, y))
    }

    pub fn get_unchecked_ref(&self, x: usize, y: usize) -> &T {
        self.array.get_unchecked_ref(self.unchecked_index_for(x, y))
    }

    pub fn get_mut_ref(&self, x: usize, y: usize) -> &mut T {
        self.array.get_unchecked_mut_ref(self.index_for(x, y))
    }

    pub fn get_unchecked_mut_ref(&self, x: usize, y: usize) -> &mut T {
        self.array.get_unchecked_mut_ref(self.unchecked_index_for(x, y))
    }
}

use std::fmt::Debug;

impl<T: Debug + Copy> Table<T> {

    pub fn print(&self){
        for y in 0..self.height {
            let mut vector = Vec::with_capacity(self.width);
            for x in 0..self.width {
                vector.push(self.get_unchecked(x, y));
            }
            println!("{:?}", vector);
        }
    }
}

impl<T: Copy> Table<T> {

    pub fn get(&self, x: usize, y: usize) -> T {
        self.array.get_unchecked(self.index_for(x, y))
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> T {
        self.array.get_unchecked(self.unchecked_index_for(x, y))
    }

    pub fn set_row(&self, y: usize, value: T){
        if y >= self.height {
            panic!("y is {} and height is {}", y, self.height);
        }
        self.set_unchecked_row(y, value);
    }

    pub fn set_unchecked_row(&self, y: usize, value: T){
        let start_index = self.unchecked_index_for(0, y);
        self.array.set_some(start_index, self.width, value);
    }

    pub fn set_column(&self, x: usize, value: T){
        if x >= self.width {
            panic!("x is {} and width is {}", x, self.width);
        }
        self.set_unchecked_column(x, value);
    }

    pub fn set_unchecked_column(&self, x: usize, value: T){
        let mut index = x;
        self.array.set_unchecked(index, value);
        for _ in 1..self.height {
            index += self.height;
            self.array.set_unchecked(index, value);
        }
    }

    pub fn set_all(&self, value: T){
        self.array.set_some(0, self.bound, value);
    }
}

impl<T: Add + AddAssign + Copy> Graphics2D<T> for Table<T> {

    fn add_unchecked(&self, x: usize, y: usize, amount: T){
        self.array.add_unchecked(self.unchecked_index_for(x, y), amount);
    }

    fn add_unchecked_rect(&self, min_x: usize, min_y: usize, max_x: usize, max_y: usize, amount: T){
        for y in min_y..=max_y {
            self.array.add_unchecked_some(self.unchecked_index_for(min_x, y), max_x - min_x + 1, amount);
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}
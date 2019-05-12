/// Some kind of array structure that uses interior mutability. It uses a Vec to claim
/// a piece of memory that will then be used to store the contents of the array.
/// The Vec will be kept as private field of the Array to make sure it won't be dropped
/// before the Array is dropped and that it will be dropped as soon as the Array is dropped.
/// 
/// Many structs of this crate will be backed by an Array.
/// 
/// There are Arrays that own their own data and Arrays that instead write to another
/// Array. Arrays that own their own data can be created with Array::new(size)
/// Arrays that write to another Array can be created by invoking the sharing_copy()
/// method of an existing Array.
/// 
/// The latter method is unsafe because undefined behavior will occur if methods of
/// the sharing copy are invoked after the original array has been dropped. It is thus
/// the responsibility of the caller to ensure that the original array lives long enough.
/// 
/// The sharing copies can be send to other threads and can thus be used to concurrently
/// modify the Array. That is another reason that the sharing_copy() method is unsafe.
/// 
/// Even though sharing accross threads is unsafe, it was the main reason to create the
/// Array struct. It is made for rare situations where performance is more important
/// than correctness.

pub struct Array<T> {

    size: usize,
    pointer: *mut T,

    _memory_owner: Option<Vec<T>>
}

unsafe impl<T> Send for Array<T> {}

impl<T> Array<T> {

    /// Creates a new Array with the given size. If the size is 0, this method will panic.
    /// The created Array will own its data.
    pub fn new(size: usize) -> Array<T> {
        if size == 0 {
            panic!("Attempted to create an array of length 0");
        }
        let mut memory_owner = Vec::with_capacity(size);
        Array {
            size: size,
            pointer: memory_owner.as_mut_ptr(),
            _memory_owner: Some(memory_owner)
        }
    }

    /// The size of this Array
    pub fn len(&self) -> usize {
        self.size
    }

    /// Checks if the given index is smaller than the size of this Array.
    /// If so, this method will return silently. If not, it will panic.
    pub fn check_bound(&self, index: usize){
        if index >= self.size {
            panic!("Index is {} and size is {}", index, self.size);
        }
    }

    /// Gets a reference to the element at the given index in this array.
    /// If the given index is not within the bounds of this array, this will panic.
    pub fn get_ref(&self, index: usize) -> &T {
        self.check_bound(index);
        unsafe {
            &*self.pointer.add(index)
        }
    }

    /// Gets a mutable reference to the element at the given index in this array.
    /// If the given index is not within the bounds of this array, this will panic.
    pub fn get_mut_ref(&self, index: usize) -> &mut T {
        self.check_bound(index);
        unsafe {
            self.pointer.add(index).as_mut().unwrap()
        }
    }

    /// Sets the element at the specified index in this array to the given value.
    /// If the given index is not within the bounds of this array, this will panic.
    pub fn set(&self, index: usize, value: T){
        self.check_bound(index);
        unsafe {
            *self.pointer.add(index) = value;
        }
    }

    /// Creates an Array instance that will share its data with this Array. This means
    /// that modifications to that Array will affect this Array and vice versa.
    /// This Array will keep owning its own data, but the returned Array will not have
    /// its own data but will use the data of this Array instead.
    /// 
    /// This method is unsafe for 2 reasons:
    /// - If this Array gets dropped before the returned Array gets dropped, invoking methods
    /// on the returned Array will manipulate data that is no longer owned and will lead to
    /// undefined behavior.
    /// - The returned Array can be sent to another thread and cause (small) concurrency problems
    /// since this struct doesn't provide any atomic mechanism.
    pub unsafe fn sharing_copy(&self) -> Array<T> {
        Array {
            size: self.size,
            pointer: self.pointer.add(0),
            _memory_owner: None
        }
    }

    pub unsafe fn sharing_sub_array(&self, start_index: usize, size: usize) -> Array<T> {
        if size == 0 {
            panic!("Size must not be 0");
        }
        self.check_bound(start_index.checked_add(size - 1).unwrap());
        Array {
            size: size,
            pointer: self.pointer.add(start_index),
            _memory_owner: None
        }
    }
}

impl<T: Copy> Array<T> {

    /// Sets some elements of this Array to (copies of) the specified value.
    /// The elements at indices start_index (inclusive) to start_index + amount (exclusive)
    /// will be set to the specified value.
    pub fn set_some(&self, start_index: usize, amount: usize, value: T){
        if amount != 0 {
            let end_index = start_index.checked_add(amount - 1).unwrap();
            self.check_bound(end_index);
            unsafe {
                for index in start_index..=end_index {
                    *self.pointer.add(index) = value;
                }
            }
        }
    }

    /// Sets all elements in this Array to (a copy of) the specified value.
    pub fn set_all(&self, value: T){
        unsafe {
            for index in 0..self.size {
                *self.pointer.add(index) = value;
            }
        }
    }

    /// Gets and returns a copy of the element at the specified index in this Array.
    pub fn get(&self, index: usize) -> T {
        self.check_bound(index);
        unsafe {
            *self.pointer.add(index)
        }
    }
}

use std::ops::AddAssign;

impl<T: AddAssign + Copy> Array<T> {

    /// Increases that element at the given index in this array by the specified amount.
    pub fn add(&self, index: usize, amount: T){
        self.check_bound(index);
        unsafe {
            *self.pointer.add(index) += amount;
        }
    }

    /// Increases some elements of this Array by the specified amount.
    /// The elements at indices start_index (inclusive) to start_index + amount (exclusive)
    /// will be increased by the specified amount.
    pub fn add_some(&self, start_index: usize, amount_of_elements: usize, amount_to_add: T){
        if amount_of_elements != 0 {
            let end_index = start_index.checked_add(amount_of_elements - 1).unwrap();
            self.check_bound(end_index);
            unsafe {
                for index in start_index..=end_index {
                    *self.pointer.add(index) += amount_to_add;
                }
            }
        }
    }

    /// Increases all elements in this Array by the specified amount.
    pub fn add_all(&self, amount: T){
        unsafe {
            for index in 0..self.size {
                *self.pointer.add(index) += amount;
            }
        }
    }
}

use num::Saturating;

impl<T: Saturating + Copy> Array<T> {

    /// Performs a saturating add on the element at the given index in this Array by the given amount.
    pub fn saturating_add(&self, index: usize, amount: T){
        self.check_bound(index);
        unsafe {
            let location = self.pointer.add(index);
            *location = (*location).saturating_add(amount);
        }
    }

    /// Performs saturating add on some elements of this Array by the specified value.
    /// The elements at indices start_index (inclusive) to start_index + amount (exclusive)
    /// will be increased.
    pub fn saturating_add_some(&self, start_index: usize, amount_of_elements: usize, amount_to_add: T){
        if amount_of_elements != 0 {
            let end_index = start_index.checked_add(amount_of_elements - 1).unwrap();
            self.check_bound(end_index);
            unsafe {
                for index in start_index..=end_index {
                    let location = self.pointer.add(index);
                    *location = (*location).saturating_add(amount_to_add);
                }
            }
        }
    }

    /// Performs a saturating addition on all elements in this Array by the given amount.
    pub fn saturating_add_all(&self, amount: T){
        unsafe {
            for index in 0..self.size {
                let location = self.pointer.add(index);
                *location = (*location).saturating_add(amount);
            }
        }
    }
}
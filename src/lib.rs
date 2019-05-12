extern crate num;

mod array;
mod table;

pub use array::Array;
pub use table::Table;

#[cfg(test)]
mod tests {

    use crate::Array;
    use crate::Table;

    #[test]
    fn test_array_basics() {
        let array = Array::new(100);

        array.set_all(74);
        assert_eq!(array.get(0), 74);
        assert_eq!(array.get(99), 74);

        array.set_some(30, 10, 63);
        assert_eq!(array.get(29), 74);
        assert_eq!(array.get(30), 63);
        assert_eq!(array.get(39), 63);
        assert_eq!(array.get(40), 74);

        array.set(99, 45);
        assert_eq!(array.get(99), 45);

        array.set_some(0, 0, 13);
        assert_eq!(array.get(0), 74);
        assert_eq!(array.get(1), 74);
        
        std::panic::catch_unwind(|| {
            array.set(100, 100);
        }).unwrap_err();
        std::panic::catch_unwind(|| {
            array.get(100);
        }).unwrap_err();
    }

    #[test]
    fn test_array_add(){
        let array: Array<u8> = Array::new(100);
        array.set_all(2);
        array.add(5, 3);
        assert_eq!(array.get(5), 5);

        array.add_some(10, 10, 7);
        assert_eq!(array.get(9), 2);
        assert_eq!(array.get(10), 9);
        assert_eq!(array.get(19), 9);
        assert_eq!(array.get(20), 2);

        array.add_all(4);
        assert_eq!(array.get(15), 13);
        assert_eq!(array.get(99), 6);

        array.saturating_add(5, 255);
        assert_eq!(array.get(5), 255);

        array.saturating_add_some(60, 5, 255);
        assert_eq!(array.get(59), 6);
        assert_eq!(array.get(60), 255);
        assert_eq!(array.get(64), 255);
        assert_eq!(array.get(65), 6);

        array.saturating_add_all(255);
        assert_eq!(array.get(65), 255);
    }

    #[test]
    fn test_array_concurrency(){

        // The task of this test is to make sure that it won't panic
        // The result of the sum is not so relevant as concurrency can affect it
        let array = Array::new(1000);
        array.set_all(0);
        let amount = 100;
        let mut handles = Vec::with_capacity(amount);
        for _ in 0..amount {
            unsafe {
                let array1 = array.sharing_copy();
            
                handles.push(std::thread::spawn(move || {
                    for index in 0..array1.len() {
                        array1.add(index, 10);
                    }
                }));
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let mut sum = 0;
        for index in 0..array.len() {
            sum += array.get(index);
        }
        println!("Sum is {}", sum);
    }

    #[test]
    fn test_table_basics(){

        // First test if no overlap occurs
        {
            let table = Table::new(Array::new(4), 2);
            table.set(0, 0, 0);
            table.set(0, 1, 1);
            table.set(1, 0, 2);
            table.set(1, 1, 3);
            assert_eq!(table.get(0, 0), 0);
            assert_eq!(table.get(0, 1), 1);
            assert_eq!(table.get(1, 0), 2);
            assert_eq!(table.get(1, 1), 3);
        }
    }
}
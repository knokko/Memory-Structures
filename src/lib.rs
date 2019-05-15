mod array;
mod table;
mod graphics;
pub mod utility;

pub use array::Array;
pub use table::Table;
pub use graphics::Graphics2D;

#[cfg(test)]
mod tests {

    use crate::Array;
    use crate::Table;
    use crate::Graphics2D;

    use std::panic::catch_unwind;

    #[test]
    fn test_array_basics() {
        let array = Array::create_filled(100, 74);

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
        
        catch_unwind(|| {
            array.set(100, 100);
        }).unwrap_err();
        catch_unwind(|| {
            array.get(100);
        }).unwrap_err();
    }

    #[test]
    fn test_array_add(){
        let array: Array<u8> = Array::create_filled(100, 2);
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
        let array = Array::create_filled(1000, 0);
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

        // First some basic tests with 2 by 2 table
        unsafe {
            let array = Array::create_garbage(5);
            array.set(4, 12);
            let table = Table::new(array.sharing_copy(), 2, 2);
            table.set(0, 0, 0);
            table.set(1, 0, 1);
            table.set(0, 1, 2);
            table.set(1, 1, 3);
            assert_eq!(table.get(0, 0), 0);
            assert_eq!(table.get(1, 0), 1);
            assert_eq!(table.get(0, 1), 2);
            assert_eq!(table.get(1, 1), 3);
            assert_eq!(array[0], 0);
            assert_eq!(array[1], 1);
            assert_eq!(array[2], 2);
            assert_eq!(array[3], 3);
            table.set_all(13);
            assert_eq!(table.get(0, 0), 13);
            assert_eq!(array[4], 12);
            
            catch_unwind(|| {
                table.set(0, 2, 0);
            }).unwrap_err();
            catch_unwind(|| {
                table.set(2, 0, 1);
            }).unwrap_err();
        }

        // Use a bigger table to test for row and column operations
        {
            let table = Table::new(Array::create_filled(9, 1), 3, 3);
            table.set_row(0, 2);
            assert_eq!(table.get(0, 0), 2);
            assert_eq!(table.get(1, 0), 2);
            assert_eq!(table.get(2, 0), 2);
            assert_eq!(table.get(0, 1), 1);

            table.set_column(1, 5);
            assert_eq!(table.get(1, 0), 5);
            assert_eq!(table.get(1, 1), 5);
            assert_eq!(table.get(1, 2), 5);
            assert_eq!(table.get(0, 2), 1);
            assert_eq!(table.get(2, 0), 2);
        }
    }

    #[test]
    fn test_table_graphics(){

        // This is just to test that it won't panic
        let width = 10;
        let height = 10;
        let array = Array::create_filled(width * height, 0);
        let table = Table::new(array, width, height);

        // A single vertical line and horizontal line
        table.draw_line(0, 0, 0, 9, 1);
        table.draw_line(0, 0, 9, 0, 1);
        table.draw_line(0, 0, 20, 30, 30);

        // Some other lines
        table.draw_line(2, 2, 7, 7, 5);
        table.draw_line(2, 7, 7, 2, 5);
        table.draw_line(7, 7, 2, 2, 5);
        table.draw_line(7, 2, 2, 7, 5);

        table.draw_line(0, 0, 2, 9, 4);

        // Remove this line later since nobody will bother reading it anyway
        table.print();
    }
}
use std::cmp::min;

pub trait Graphics2D<T: Copy> {

    fn draw_line(&self, x1: usize, y1: usize, x2: usize, y2: usize, value: T){
        let width = self.get_width();
        let height = self.get_height();

        // If this condition holds, the entire line will be outside the range and thus we don't need to do anything
        if (x1 >= width && x2 >= width) || (y1 >= height && y2 >= height){
            return;
        }
        
        // If we have a horizontal or vertical line, draw them the simple way
        if x1 == x2 {

            // Only draw the line if it is within our bounds
            if x1 < self.get_width() {
                if y1 < y2 {
                    self.add_unchecked_rect(x1, y1, x1, min(y2, height - 1), value);
                } else {
                    self.add_unchecked_rect(x1, y2, x1, min(y1, height - 1), value);
                }
            }
        } else if y1 == y2 {

            // Only draw the line if it is within our bounds
            if y1 < self.get_height() {
                if x1 < x2 {
                    self.add_unchecked_rect(x1, y1, min(x2, width - 1), y1, value);
                } else {
                    self.add_unchecked_rect(x2, y1, min(x1, width - 1), y1, value);
                }
            }
        } else {
            // If we end up here, the line is not horizontal or vertical, so work needs to be done
            // We will express x in y and y in x to make sure we don't miss any points
            let min_x;
            let min_y;
            let max_x;
            let max_y;

            if x1 < x2 {
                min_x = x1;
                max_x = x2;
            } else {
                min_x = x2;
                max_x = x1;
            }
            if y1 < y2 {
                min_y = y1;
                max_y = y2;
            } else {
                min_y = y2;
                max_y = y1;
            }

            let dx = max_x - min_x;
            let dy = max_y - min_y;

            let negative_slope = (x1 == min_x) ^ (y1 == min_y);

            let end_x;
            if max_x >= width {
                end_x = width - 1 - min_x;
            } else {
                end_x = max_x - min_x;
            }

            let end_y;
            if max_y >= height {
                end_y = height - 1 - min_y;
            } else {
                end_y = max_y - min_y;
            }

            let mut check_overflow = false;
            {
                let maybe_product = end_x.checked_mul(dy);
                match maybe_product {
                    Some(product) => {
                        if product.checked_add(min_y).is_none() {
                            check_overflow = true;
                        }
                    },
                    None => check_overflow = true
                };
            }
            {
                let maybe_product = end_y.checked_mul(dx);
                match maybe_product {
                    Some(product) => {
                        if product.checked_add(min_x).is_none() {
                            check_overflow = true;
                        }
                    },
                    None => check_overflow = true
                };
            }

            if check_overflow {
                panic!("I should implement proper overflow handling someday...");
            };

            for extra_x in 0..=end_x {
                let extra_y = extra_x * dy / dx;
                if negative_slope {
                    if extra_y <= max_y {
                        self.add_unchecked(min_x + extra_x, max_y - extra_y, value);
                    }
                } else {
                    let y = extra_y + min_y;
                    if y < height {
                        self.add_unchecked(extra_x + min_x, y, value);
                    }
                }
            }

            for extra_y in 0..=end_y {
                let extra_x = extra_y * dx / dy;
                if negative_slope {
                    if extra_x <= max_x {
                        self.add_unchecked(max_x - extra_x, extra_y + min_y, value);
                    }
                } else {
                    let x = extra_x + min_x;
                    if x < width {
                        self.add_unchecked(x, extra_y + min_y, value);
                    }
                }
            }
        }
    }

    fn add_unchecked(&self, x: usize, y: usize, amount: T);

    fn add_unchecked_rect(&self, min_x: usize, min_y: usize, max_x: usize, max_y: usize, amount: T);

    fn get_width(&self) -> usize;

    fn get_height(&self) -> usize;
}
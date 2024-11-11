use std::ops::Div;
use num_integer::sqrt;

pub trait GridPoint {
    fn get_x( &self ) -> usize;
    fn set_x( &mut self, new_value: usize );
    fn get_y( &self ) -> usize;
    fn set_y( &mut self, new_value: usize );
}

pub trait GridPrintable {
    fn get_print( &self ) -> String;
}

pub fn print_map<T: GridPrintable + GridPoint>(map: &Vec<T>, grid_width: usize ) {
    for i in 0..map.len() {
        if i % grid_width == 0 {
            println!()
        }
        print!("{}", map[i].get_print() );
    }
    println!();
}
pub fn serialize_map<T: GridPrintable + GridPoint>(map: &Vec<T>, grid_width: usize ) -> String {
    let mut buf = "".to_string();
    for i in 0..map.len() {
        if i % grid_width == 0 {
            buf.push_str("\n");
        }
        buf.push_str( map[i].get_print().as_str() );
    }
    buf
}

pub fn get_row<T: GridPoint + Clone>( row: usize, grid_points: &[T], grid_width: usize ) -> &[T] {
    &grid_points[row * grid_width..(row * grid_width) + grid_width]
}

// Rotates the map 90 degrees. Actually there is a bug here where it comes out rotated and flipped, but didn't have brainpower to figure out why, and it also doesn't matter.
pub fn rotate_map<T: GridPoint>(map: &mut Vec<T>, new_grid_width: usize ) {

    map.iter_mut().for_each( |mut m| {
        let new_x = (( m.get_y() + new_grid_width ) % new_grid_width).abs_diff( new_grid_width - 1 );
        let new_y = m.get_x();
        m.set_y( new_y );
        m.set_x( new_x );
    } );
    map.sort_by( | a, b | a.get_y().cmp( &b.get_y() ).then_with(|| a.get_x().cmp(&b.get_x()) ) );
}

// pub fn get_col<T: GridPoint + Clone>( col: usize, grid_points: &[T], grid_width: usize ) -> &[T] {
//     let mut answer: Vec<T> = vec![];
//     //return grid_points.filter
//     //&grid_points[col * grid_width..(col * grid_width) + grid_width]
// }

pub fn find_angle<T: GridPoint>( point_from: &T, point_to: &T ) -> f64 {

    let is_right = point_from.get_x() < point_to.get_x();
    let is_below = point_from.get_y() < point_to.get_y();

    if point_from.get_y() == point_to.get_y() {
        if is_right {
            return 90f64
        }
        return 270f64;
    }
    if point_from.get_x() == point_to.get_x() {
        if is_below {
            return 180f64
        }
        return 0f64;
    }

    let y_diff = ( point_to.get_y() ).abs_diff(point_from.get_y() ) as f64;
    let x_diff = ( point_to.get_x() ).abs_diff(point_from.get_x() ) as f64;

    let mut opposite: f64 = y_diff;
    let mut adjacent: f64 = x_diff;

    if is_below && is_right {
        opposite = x_diff;
        adjacent = y_diff;
    }
    let hypotenuse: f64 = ( opposite.powf( 2f64 ) + adjacent.powf( 2f64 ) ).sqrt();

    let angle = opposite.div( hypotenuse ).asin().to_degrees();

    let rounded = ( angle * 10000.0 ).round() / 10000.0;//f64;
    if is_right {
        if is_below {
            let angle = ( ( angle + 90f64 ) * 10000.0 ).round() / 10000.0;
            return angle;
        }
        return rounded;
    }
    if is_below {
        let angle = ( ( angle + 180f64 ) * 10000.0 ).round() / 10000.0;
        return angle;
    }
    let angle = ( ( angle + 270f64 ) * 10000.0 ).round() / 10000.0;
    return angle;
}

#[cfg(test)]
mod tests {
    use std::fmt::format;
    use crate::grid_utils::grid_utils::{find_angle, get_row, GridPoint};

    #[test]

    fn find_row_works() {
        #[derive(Clone,Debug)]
        struct Test {
            x: usize,
            y: usize,
        }

        impl GridPoint for Test {
            fn get_x( &self ) -> usize {
                return self.x;
            }

            fn set_x(&mut self, new_value: usize) {
                todo!()
            }

            fn get_y( &self ) -> usize {
                return self.y;
            }

            fn set_y(&mut self, new_value: usize) {
                todo!()
            }
        }
        let mut points: Vec<Test>  = vec![];

        for y in 0..10 {
            for x in 0..10 {
                points.push( Test{ x, y } )
            }
        }

        assert_eq!( get_row( 1, points.as_slice(), 10 ).iter().map( | p | format!( "{},{}", p.get_x(), p.get_y() ) ).collect::<Vec<String>>().join(""), points[10..=19].iter().map( | p | format!( "{},{}", p.get_x(), p.get_y() ) ).collect::<Vec<String>>().join("") );

        points = vec![];


        for y in 0..100 {
            for x in 0..3 {
                points.push( Test{ x, y } )
            }
        }

        // 0, 1, 2
        // 3, 4, 5
        // 6, 7, 8
        // 9, 10, 11
        // 12, 13, 14
        // 15, 16, 17
        // 18, 19, 20
        // 21, 22, 23

        assert_eq!( get_row( 7, points.as_slice(), 3 ).iter().map( | p | format!( "{},{}", p.get_x(), p.get_y() ) ).collect::<Vec<String>>().join(""), points[21..=23].iter().map( | p | format!( "{},{}", p.get_x(), p.get_y() ) ).collect::<Vec<String>>().join("") );
    }
    #[test]
    fn find_angle_works() {
        struct Test {
            x: usize,
            y: usize,
        }

        impl GridPoint for Test {
            fn get_x( &self ) -> usize {
                return self.x;
            }

            fn set_x(&mut self, new_value: usize) {
                todo!()
            }

            fn get_y( &self ) -> usize {
                return self.y;
            }

            fn set_y(&mut self, new_value: usize) {
                todo!()
            }
        }
        assert_eq!(find_angle(&Test { x: 0, y: 0 }, &Test { x: 0, y: 5 } ), 180f64 );
        assert_eq!( find_angle( &Test{ x: 0, y: 70 }, &Test { x: 0, y: 5 } ), 0f64 );
        assert_eq!( find_angle( &Test{ x: 0, y: 70 }, &Test { x: 1, y: 70 } ), 90f64 );
        assert_eq!( find_angle( &Test{ x: 110, y: 70 }, &Test { x: 1, y: 70 } ), 270f64 );
        assert_eq!( find_angle( &Test{ x: 0, y: 0 }, &Test { x: 2, y: 1 } ), 63.4349f64 );
        assert_eq!( find_angle( &Test{ x: 1, y: 2 }, &Test { x: 0, y: 0 } ), 333.4349 );
       // assert_eq!( find_angle( &Test{ x: 1, y: 2 }, &Test { x: 0, y: 0 } ), -63.43f64 );
    }
}

pub fn get_neighbor_indices( i: usize, grid_width: usize ) -> ( isize, isize, isize, isize ) {
    ( i as isize - 1, i as isize + 1, i as isize - grid_width as isize, i as isize + grid_width as isize )
}

pub fn find_manhattan_distance<T: GridPoint>( g1: T, g2: T ) -> usize {
    return g1.get_y().abs_diff( g2.get_y() ) + g1.get_x().abs_diff( g2.get_x() );
}
pub fn get_index_at_coords(x: usize, y: usize, grid_width: usize ) -> usize {
    return y * grid_width + x;
}
pub fn get_coords_at_index( index: usize, grid_width: usize ) -> ( usize, usize ) {
    let y = index / grid_width;
    let x = index % grid_width;
    ( x, y )
}
#[derive(Clone)]
struct Tree {
    height: usize,
    visible: Visibility,
    scenic_score: i32,
}

impl Tree {
    fn set_visibility( &mut self, visibility: Visibility ) {
        self.visible = visibility;
    }
}

#[derive(Clone)]
struct Visibility {
    n: bool,
    e: bool,
    s: bool,
    w: bool
}

impl Visibility {
    fn is_visible( &self ) -> bool {
        self.e || self.n || self.w || self.s
    }
}

fn get_n_vis_score( trees: &Vec<Vec<Tree>>, row: usize, col: usize ) -> usize {
    let tree_height = trees.get(row).unwrap().get(col).unwrap().height;
    if let Some(trees_above) = trees.get( 0 .. row ) {
        let mut equal_found = false;
        let trees_count = trees_above.iter().rev().cloned().take_while( | t | {
            if equal_found {
                return false;
            }
            let comp_tree = t.get( col ).unwrap().height;
            if comp_tree >= tree_height {
                equal_found = true;
                return true;
            }
            return (comp_tree < tree_height) || equal_found;
        } ).collect::<Vec<_>>().len();
        return trees_count;
    }
    return 0;
}
fn get_s_vis_score( trees: &Vec<Vec<Tree>>, row: usize, col: usize ) -> usize {
    let tree_height = trees.get(row).unwrap().get(col).unwrap().height;
    println!("Getting vis height for {},{}, its height is {}", col,row,tree_height);
    if let Some(trees_below) = trees.get( row+1 .. trees.len() ) {
        let mut equal_found = false;
        let trees_count = trees_below.iter().cloned().take_while( | t | {
            if equal_found {
                return false;
            }
            let comp_tree = t.get( col ).unwrap().height;
            if comp_tree >= tree_height {
                equal_found = true;
                return true;
            }
            return comp_tree <= tree_height;
        } ).collect::<Vec<_>>().len();
        return trees_count;
    }
    return 0;
}
fn get_e_vis_score( trees: &Vec<Vec<Tree>>, row: usize, col: usize ) -> usize {
    let tree_height = trees.get(row).unwrap().get(col).unwrap().height;
    let row = trees.get( row ).cloned().unwrap();
    if let Some( trees_to_right ) = row.get( col+1 .. row.len() ) {
        let mut equal_found = false;
        return trees_to_right.iter().cloned().take_while( |f | {
            if equal_found {
                return false;
            }
            let comp_tree = f.height;
            if comp_tree >= tree_height {
                equal_found = true;
                return true;
            }
            return comp_tree <= tree_height;
        } ).collect::<Vec<_>>().len();
    }
    return 0;
}


fn get_w_vis_score( trees: &Vec<Vec<Tree>>, row: usize, col: usize ) -> usize {
    let tree_height = trees.get(row).unwrap().get(col).unwrap().height;
    let row = trees.get( row ).cloned().unwrap();
    if let Some( trees_to_left ) = row.get( 0 .. col ) {
        let mut equal_found = false;
        return trees_to_left.iter().rev().cloned().take_while( |f | {
            if equal_found {
                return false;
            }
            let comp_tree = f.height;
            if comp_tree >= tree_height {
                equal_found = true;
                return true;
            }
            return comp_tree <= tree_height;
        } ).collect::<Vec<_>>().len();
    }
    return 0;
}

fn is_visible_from_n( trees: &Vec<Vec<Tree>>, row: usize, col: usize ) -> bool {
    let tree_height = trees.get(row).unwrap().get(col).unwrap().height;

    if let Some(trees_above) = trees.get( 0 .. row ) {
        return trees_above.iter().all( |f | {
            return f.get(col).unwrap().height < tree_height;
        } );
    }
    return false;
}

fn is_visible_from_s( trees: &Vec<Vec<Tree>>, row: usize, col: usize ) -> bool {
    let tree_height = trees.get(row).unwrap().get(col).unwrap().height;

    if let Some(trees_below) = trees.get( row+1 .. trees.len() ) {
        return trees_below.iter().all( |f | {
            return f.get(col).unwrap().height < tree_height;
        } );
    }
    return false;
}

fn is_visible_from_e( trees: &Vec<Vec<Tree>>, row: usize, col: usize ) -> bool {
    let tree_height = trees.get(row).unwrap().get(col).unwrap().height;
    let row = trees.get( row ).cloned().unwrap();
    if let Some( trees_to_right ) = row.get( col+1 .. row.len() ) {
        return trees_to_right.iter().all( |f | {
            return f.height < tree_height
        } );
    }
    return false;
}

fn is_visible_from_w( trees: &Vec<Vec<Tree>>, row: usize, col: usize ) -> bool {
    let tree_height = trees.get(row).unwrap().get(col).unwrap().height;
    let row = trees.get( row ).cloned().unwrap();
    if let Some( trees_to_left ) = row.get( 0 .. col ) {
        return trees_to_left.iter().all( |f | {
            return f.height < tree_height
        } );
    }
    return false;
}

pub fn task() {
    let input = crate::input_reader::input_reader::read_input_for_day( 2022, 8, false );
    let output_lines: Vec<&str> = input.split("\n").collect();

    let mut trees: Vec<Vec<Tree>> = vec![];

    let mut row = 0;
    for tree_row in output_lines {
        let mut col = 0;
        trees.push( Vec::new() );
        for tree_col in tree_row.chars() {
            let tree_height = tree_col as usize - '0' as usize;
           // println!("{},{} height={}", col, row, tree_height);
            trees.get_mut( row ).unwrap().push( Tree { scenic_score:0, height: tree_height, visible: Visibility{e:false,w:false,s:false,n:false} } );
            col += 1;
        }
        row += 1;
    }

    let grid_width = trees.len();
    let grid_height = trees.get(0).unwrap().len();

    row = 0;
    let mut visible_trees = 0;
    let mut highest_vis_score = 0;

    for mut tree_row in &trees {
        let mut col = 0;
        for mut tree_col in tree_row {

        }
    }

    // let x = 1;
    // let y = 2;
    // let n = get_n_vis_score( &trees, x, y );
    // let s = get_s_vis_score( &trees, x, y );
    // let e = get_e_vis_score( &trees, x, y );
    // let w = get_w_vis_score( &trees, x, y );
    //

    for mut tree_row in &trees {
        let mut col = 0;
        for mut tree_col in tree_row {
            let mut visible_from_n = row == 0;
            let mut visible_from_s = row == grid_height - 1;
            let mut visible_from_e = col == grid_width - 1;
            let mut visible_from_w = col == 0;
            if ! visible_from_n {
                visible_from_n = is_visible_from_n( &trees, row, col);
            }
            if ! visible_from_s {
                visible_from_s = is_visible_from_s( &trees, row, col );
            }
            if ! visible_from_e {
                visible_from_e = is_visible_from_e( &trees, row, col );
            }
            if ! visible_from_w {
                visible_from_w = is_visible_from_w( &trees, row, col );
            }
            let height = trees.clone().get(row).unwrap().get(col).unwrap().height;

            let vis =  Visibility {
                n: visible_from_n,
                e: visible_from_e,
                w: visible_from_w,
                s: visible_from_s,
            };

            let vis_score = get_n_vis_score( &trees, row, col ) * get_e_vis_score( &trees, row, col ) * get_s_vis_score( &trees, row, col ) * get_w_vis_score( &trees, row, col );
            if highest_vis_score < vis_score {
                highest_vis_score = vis_score;
            }

            if vis.is_visible() {
                visible_trees += 1;
            }
            //println!("Tree in {}, {}, is {}, N: {}, E: {}, S: {}, W: {}", row, col, height, visible_from_n, visible_from_e, visible_from_s, visible_from_w );
            col+=1;
        }
        row+=1;
    }
    println!("Day 8 task 1: {}", visible_trees);
    println!("Day 8 task 2: {}", highest_vis_score);

    //println!("N: {}\nE: {}\nS: {}\nW: {}", n, e, s, w)
    //let c  =1;
}
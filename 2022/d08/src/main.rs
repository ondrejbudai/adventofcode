use std::io::{BufRead, stdin};

fn main() {
    let mut forest_rows: Vec<Vec<i8>> = Vec::new();
    stdin().lock().lines().for_each(|input_result| {
        let result = input_result.unwrap();
        let mut row: Vec<i8> = Vec::new();
        result.bytes().for_each(|tree| {
            row.push(tree as i8 - ('0' as i8));
        });
        forest_rows.push(row);
    });

    let mut visible = 0;
    let mut best_scenic_score = 0;

    forest_rows.iter().enumerate().for_each(|(row, cols)| {
        for (col, height) in cols.iter().enumerate() {
            let mut side_hidden = 0u16;
            let mut distances: [u16; 4] = [0; 4];
            for col1 in (0..col).rev() {
                distances[0] += 1;
                if *cols.get(col1 as usize).unwrap() >= *height {
                    side_hidden += 1;
                    break;
                }
            }

            for col1 in col + 1..cols.len() {
                distances[1] += 1;
                if *cols.get(col1).unwrap() >= *height {
                    side_hidden += 1;
                    break;
                }
            }

            for row1 in (0..row).rev() {
                distances[2] += 1;
                if *forest_rows.get(row1 as usize).unwrap().get(col).unwrap() >= *height {
                    side_hidden += 1;
                    break;
                }
            }

            for row1 in row + 1..forest_rows.len() {
                distances[3] += 1;
                if *forest_rows.get(row1).unwrap().get(col).unwrap() >= *height {
                    side_hidden += 1;
                    break;
                }
            }

            if side_hidden < 4 {
                visible += 1;
            }

            let scenic_score = distances.iter().fold(1u32, |cur_scenic_score, height| {
                cur_scenic_score * *height as u32
            });
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    });

    // let mut current_col_heights = vec![-1i8; forest_rows.len()];
    // forest_rows.iter().for_each(|row| {
    //     let mut current_row_height: i8 = -1;
    //
    //     row.iter().enumerate().for_each(|(col, height)| {
    //         let col_height = current_col_heights.get_mut(col).unwrap();
    //         if *height > current_row_height {
    //             current_row_height = *height;
    //             visible += 1;
    //         } else if *height > *col_height {
    //             *col_height = *height;
    //             visible += 1;
    //         }
    //     });
    //
    //     current_row_height = -1;
    //     row.iter().rev().for_each(|height| {
    //         if *height > current_row_height {
    //             current_row_height = *height;
    //             visible += 1;
    //         }
    //     });
    // });
    //
    // let mut current_col_heights = vec![-1i8; forest_rows.len()];
    // forest_rows.iter().rev().for_each(|row| {
    //     row.iter().enumerate().for_each(|(col, height)| {
    //         let col_height = current_col_heights.get_mut(col).unwrap();
    //         if *height > *col_height {
    //             *col_height = *height;
    //             visible += 1;
    //         }
    //     });
    //
    // });

    println!("{visible}");
    println!("{best_scenic_score}");
}

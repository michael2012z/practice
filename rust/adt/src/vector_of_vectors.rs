// https://leetcode.com/problems/pascals-triangle/
fn pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    let mut pascal: Vec<Vec<i32>> = vec![];
    for i in 1..((num_rows + 1) as usize) {
        let mut row: Vec<i32> = vec![];
        for j in 0..i {
            if j == 0 || j == (i - 1) {
                row.push(1i32)
            } else {
                row.push(pascal[i - 2][j - 1] + pascal[i - 2][j])
            }
        }
        pascal.push(row)
    }
    pascal
}

pub fn test() {
    let vec_of_vec = pascal_triangle(10);
    /*
    for vec in vec_of_vec {
        for i in vec {
            print!("{i} ");
        }
        println!("");
    }
    */
    assert_eq!(vec_of_vec.len(), 10);
}

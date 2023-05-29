pub fn test() {
    let x = [[0i32; 5]; 5];
    //println!("{:#?}", x);
    assert_eq!(x.len(), 5);
    let mut y = [[0i32; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            y[i][j] = (i + j) as i32;
        }
    }
    assert_eq!(y.len(), 5);
    assert_eq!(y[0].len(), 5);
    //println!("{:#?}", y);
}

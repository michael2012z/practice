const BOARDSIZE: usize = 6;

// Knight's Tour puzzle:
// https://en.wikipedia.org/wiki/Knight%27s_tour
pub fn test() {
    println!("Board size: {BOARDSIZE}");
    for i in 0..BOARDSIZE {
        for j in 0..BOARDSIZE {
            // Board
            //   0: Non-visited
            //   1: Visited
            let mut board = [[0u8; BOARDSIZE]; BOARDSIZE];
            let mut counter = 0;
            if try_position(&mut board, i as i32, j as i32, 1, &mut counter) {
                println!("Starting position: ({i}, {j}), trials: {counter}");
            } else {
                println!("Starting position ({i}, {j}) is impossible");
            }
        }
    }
}

fn try_position(
    board: &mut [[u8; BOARDSIZE]],
    x: i32,
    y: i32,
    depth: usize,
    counter: &mut usize,
) -> bool {
    *counter = *counter + 1;
    if board[x as usize][y as usize] == 1 {
        // Visited, go back
        false
    } else {
        if depth == BOARDSIZE * BOARDSIZE {
            // Succeed
            true
        } else {
            board[x as usize][y as usize] = 1;
            // Try every possible next position
            let candidates = [
                (x + 1, y + 2),
                (x + 2, y + 1),
                (x + 2, y - 1),
                (x + 1, y - 2),
                (x - 1, y - 2),
                (x - 2, y - 1),
                (x - 2, y + 1),
                (x - 1, y + 2),
            ];
            for candidate in candidates {
                let (xx, yy) = candidate;
                if xx >= 0 && xx < BOARDSIZE as i32 && yy >= 0 && yy < BOARDSIZE as i32 {
                    if try_position(board, xx, yy, depth + 1, counter) {
                        // Find a working candidate
                        // println!("({}, {})", xx, yy);
                        return true;
                    }
                }
            }
            // There is no working candidate, fall back
            board[x as usize][y as usize] = 0;
            false
        }
    }
}

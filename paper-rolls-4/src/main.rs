struct Neighbours {
    a: &'static str,
    b: String,
    c: String,
    m: String,
    n: String,
    p: String,
    r: String,
    q: String,
}

struct Matrix {
    elem: Vec<Vec<&'static str>>,
}

impl Matrix {
    fn new(rows: Vec<&str>) -> Self {
        let mut matrix = Vec::new();
        for row in 0..rows.len() {
            matrix[row] = rows[row]
                .chars()
                .map(|c| c.to_string().as_ref())
                .collect::<Vec<&str>>();
        }
        Matrix { elem: matrix }
    }

    fn neighbours(&self, idx: (usize, usize)) -> Neighbours {
        let (x, y) = idx;
        Neighbours {
            a: self.elem[x - 1][y - 1],
            b: self.elem[x - 1][y],
            c: self.elem[x - 1][y + 1],
            m: self.elem[x][y - 1],
            n: self.elem[x][y + 1],
            p: self.elem[x + 1][y - 1],
            q: self.elem[x + 1][y],
            r: self.elem[x + 1][y + 1],
        }
    }
}

fn main() {
    let lines: Vec<&str> = vec![
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    let mut matrix: Vec<Vec<String>> = Vec::new();

    // Prepare the matrix.
    for row in 0..(lines.first().unwrap().len()) {
        matrix[row] = lines[row]
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
    }

    for row in 0..matrix.len() {
        // row
        for col in 0..matrix.first().unwrap().len() {
            // col
            let x = (row - 1, col - 1);
            let y = (row - 1, col);
            let z = (row - 1, col + 1);
            let m = (row, col - 1);
        }
    }
}

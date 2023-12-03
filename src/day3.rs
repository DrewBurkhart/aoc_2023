pub fn problem1() {
    let input = std::fs::read_to_string("inputs/input3.txt").expect("should've been able to read");
    let (x_dimension, y_dimension, matrix) = create_2d_char_vector(&input);
    let adjacent_symbol = identify_symbols(&matrix, x_dimension, y_dimension);
    println!(
        "{}",
        sum(&matrix, x_dimension, y_dimension, &adjacent_symbol)
    );
}

fn sum(
    matrix: &Vec<Vec<char>>,
    x_dimension: usize,
    y_dimension: usize,
    symbols: &Vec<Vec<bool>>,
) -> u64 {
    let mut sum = 0;
    let mut curr: u64 = 0;
    let mut had_flag = false;
    for x in 0..x_dimension {
        for y in 0..y_dimension {
            let element = matrix[x][y];
            if element == '.' || !element.is_digit(10) && element != '.' {
                if had_flag {
                    sum += curr;
                }
                curr = 0;
                had_flag = false;
                continue;
            }
            had_flag = had_flag || symbols[x][y];
            if element.is_digit(10) {
                curr = curr * 10 + element.to_digit(10).unwrap() as u64;
                continue;
            }
        }
    }
    sum
}

fn create_2d_char_vector(map: &str) -> (usize, usize, Vec<Vec<char>>) {
    let mut input: Vec<Vec<char>> = map
        .lines()
        .map(|l| {
            let l = format!("..{}..", l);
            l.chars().collect::<Vec<char>>()
        })
        .collect();
    let empty_line: Vec<char> = (0..input[0].len()).map(|_i| '.').collect();
    input.insert(0, empty_line.clone());
    input.push(empty_line);
    (input.len(), input[0].len(), input)
}

fn identify_symbols(
    matrix: &Vec<Vec<char>>,
    x_dimension: usize,
    y_dimension: usize,
) -> Vec<Vec<bool>> {
    let mut output = vec![vec![false; y_dimension]; x_dimension];
    for x in 0..x_dimension - 1 {
        for y in 0..y_dimension - 1 {
            if !matrix[x][y].is_digit(10) && matrix[x][y] != '.' {
                for i in x - 1..=x + 1 {
                    for j in y - 1..=y + 1 {
                        output[i][j] = true;
                    }
                }
            }
        }
    }
    output
}

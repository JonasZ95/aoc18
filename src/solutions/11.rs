use std::isize;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
const N: usize = 301;
type Mat2 = [[isize; N]; N];

fn print_mat2(mat2: &Mat2) {
    for y in 0..N {
        for x in 0..N {
            print!("{} ", mat2[y][x]);
        }
        println!()
    }
}

fn build_grid(serial: usize) -> Mat2 {
    let mut grid = [[0; N]; N];

    for y in 1..N {
        for x in 1..N {
            let mut rack_id = x+10;

            let mut power = rack_id * y;
            power += serial;
            power *= rack_id;
            power = (power / 100) % 10;
            grid[y][x] = (power as isize) - 5;
        }
    }
    grid
}

fn build_sum_table(grid: &Mat2) -> Mat2 {
    let mut sum_table = [[0; N]; N];
    for y in 1..N {
        for x in 1..N {
            let mut power = grid[y][x];
            sum_table[y][x] = power +
                sum_table[y-1][x] +
                sum_table[y][x-1] -
                sum_table[y-1][x-1];
        }
    }

    sum_table
}

fn calc_sum(sum_table: &Mat2, x: usize, y: usize, m: usize) -> isize {
    let a = sum_table[y][x];
    let b = sum_table[y][x-m];
    let c = sum_table[y-m][x];
    let d = sum_table[y-m][x-m];

    d+a-b-c
}

fn find_max(sum_table: &Mat2, m: usize) -> (usize, usize, isize) {
    let (mut max, mut max_x, mut max_y) = (isize::MIN, 0, 0);

    for y in m..N {
        for x in m..N {
            let sum = calc_sum(&sum_table, x, y, m);

            if sum > max {
                max = sum;
                max_x = x;
                max_y = y;
            }
        }
    }
    (max_x-m+1, max_y-m+1, max)
}

fn part1(serial: usize) -> Result<(usize, usize)> {
    let grid = build_grid(serial);
    let sum_table = build_sum_table(&grid);

    let (x, y, _) = find_max(&sum_table, 3);
    Ok((x, y))

}

fn part2(serial: usize) -> Result<(usize, usize, usize)> {
    let grid = build_grid(serial);
    let sum_table = build_sum_table(&grid);

    let (x, y, _, m) = (1..N)
        .map(|m| {
            let (x, y, max) = find_max(&sum_table, m);
            (x, y, max, m)
        })
        .max_by_key(|(_, _, max, _)| *max)
        .unwrap();
    Ok((x, y, m))
}

fn main() -> Result<()> {
    let sample1 = part1(18)?;
    let sample2 = part2(18)?;
    println!("sample 1: {:?}, sample2: {:?}", sample1, sample2);

    let result1 = part1(4172)?;
    let result2 = part2(4172)?;
    println!("#1: {:?}, 2: {:?}", result1, result2);
    Ok(())
}
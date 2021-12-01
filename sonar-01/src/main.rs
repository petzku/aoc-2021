use std::fs;

fn main() {
    let f = "sonar.input.txt";
    let depths = read_depths_from_file(f);

    // part 1
    let incs = count_depth_increases(&depths);
    println!("increased {} times", incs);

    // part 2
    let incs = count_depth_increases(&sliding_windows(&depths));
    println!("window increased {} times", incs)
}

fn count_depth_increases(depths: &Vec<u64>) -> u64 {
    let mut n = 0;
    let mut prev = &depths[0];
    for x in depths {
        if x > prev {
            n += 1;
        }
        prev = x;
    }
    n
}

fn read_depths_from_file(filename: &str) -> Vec<u64> {
    let contents = fs::read_to_string(filename)
        .expect("error while reading file");

    contents.split_whitespace().map(|s| s.parse().expect("parse error")).collect()
}

fn sliding_windows(v: &Vec<u64>) -> Vec<u64> {
    v.iter()
        .zip(&v[1..])
        .zip(&v[2..])
        .map( |((a,b), c)| a+b+c)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_depths() {
        let depths = vec![ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];
        assert_eq!(count_depth_increases(&depths), 7);
    }

    #[test]
    fn simple_sliding() {
        let depths = vec![ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];
        let sums = sliding_windows(&depths);
        assert_eq!(sums, vec![ 607, 618, 618, 617, 647, 716, 769, 792 ]);
    }

    #[test]
    fn sliding_compare() {
        let depths = vec![ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];
        assert_eq!(count_depth_increases(&sliding_windows(&depths)), 5);
    }
}

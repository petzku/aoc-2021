use std::fs;

fn main() {
    let f = "sonar.input.txt";
    let depths = read_depths_from_file(f);
    let incs = count_depth_increases(depths);
    println!("increased {} times", incs);
}

fn count_depth_increases(depths: Vec<u64>) -> u64 {
    let mut n = 0;
    let mut prev = &depths[0];
    for x in &depths {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_depths() {
        let depths = vec![ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];
        assert_eq!(count_depth_increases(depths), 7);
    }
}

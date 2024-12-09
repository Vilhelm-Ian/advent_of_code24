fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

#[derive(Clone)]
enum Data {
    FreeDisk,
    File(i64),
}

pub fn solve(input: &str) -> i64 {
    let mut result = 0;
    let mut disk = parse(input);
    let mut i = 0;
    loop {
        if i >= disk.len() {
            break;
        }
        let size = match disk[i] {
            Data::File(size) => size,
            Data::FreeDisk => {
                let mut size = None;
                while let Some(data) = disk.pop() {
                    if let Data::File(s) = data {
                        size = Some(s);
                        break;
                    };
                }
                size.unwrap()
            }
        };
        result += size * i as i64;
        i += 1;
    }
    result
}

fn parse(mut input: &str) -> Vec<Data> {
    input = input.trim();
    let mut result = vec![];
    let mut id = 0;
    for (i, char) in input.chars().enumerate() {
        let amount = char.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            let files = vec![Data::File(id); amount];
            result.extend_from_slice(&files);
            id += 1;
        } else {
            let free = vec![Data::FreeDisk; amount];
            result.extend_from_slice(&free);
        }
    }
    result
}

fn disk_to_str(disk: &Vec<Data>) -> String {
    let mut result = String::from("");
    for data in disk {
        let postfix = match data {
            Data::FreeDisk => ".".to_string(),
            Data::File(file) => file.to_string(),
        };
        result += postfix.as_str();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "2333133121414131402";
        let result = solve(input);
        assert_eq!(result, 1928);
    }

    #[test]
    fn print() {
        let input = "2333133121414131402";
        let input = parse(input);
        let result = disk_to_str(&input);
        assert_eq!(result, "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn print_2() {
        let input = "12345";
        let input = parse(input);
        let result = disk_to_str(&input);
        assert_eq!(result, "0..111....22222");
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

struct File {
    id: i64,
    value: i64,
}

impl File {
    fn new(value: i64, id: i64) -> Self {
        Self { id, value }
    }
}

enum Data {
    FreeDisk(i64),
    File(File),
}

fn solve(input: &str) -> i64 {
    let disk = parse(input);
    let mut disk: Vec<char> = disk_to_str(disk).chars().collect();
    let mut i = 0;
    let mut result = 0;
    loop {
        if i >= disk.len() {
            break;
        }
        let mut current = i64::MAX;
        if disk[i] == '.' {
            while let Some(data) = disk.pop() {
                if data != '.' {
                    current = data.to_digit(10).unwrap() as i64;
                    break;
                }
            }
        } else {
            current = disk[i].to_digit(10).unwrap() as i64;
        }
        result += current * i as i64;
        i += 1;
    }
    result
}

fn parse(mut input: &str) -> Vec<Data> {
    input = input.trim();
    let mut result = vec![];
    let mut id = 0;
    for (i, char) in input.chars().enumerate() {
        if i % 2 == 0 {
            result.push(Data::File(File::new(char.to_digit(10).unwrap() as i64, id)));
            id += 1;
        } else {
            result.push(Data::FreeDisk(char.to_digit(10).unwrap() as i64));
        }
    }
    result
}

fn disk_to_str(disk: Vec<Data>) -> String {
    let mut result = String::from("");
    for data in disk {
        let postfix = match data {
            Data::FreeDisk(amount) => ".".repeat(amount as usize),
            Data::File(file) => (file.id.to_string()).repeat(file.value as usize),
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
        let result = disk_to_str(input);
        assert_eq!(result, "00...111...2...333.44.5555.6666.777.888899");
    }
}

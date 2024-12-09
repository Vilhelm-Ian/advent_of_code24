use std::collections::{BTreeMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = solve(input);
    println!("{:?}", result);
}

#[derive(Clone)]
enum Data {
    FreeDisk(usize),
    File(i64, usize),
}

pub fn solve(input: &str) -> i64 {
    let mut result = 0;
    let (mut disk, mut files) = parse(input);
    let mut seen = HashSet::new();

    let mut i = 0;
    let mut z = 0;
    loop {
        if i >= disk.len() {
            break;
        }
        match disk[i] {
            Data::File(id, amount) => {
                if !seen.contains(&id) {
                    update_result(&id, amount, &mut z, &mut result);
                } else {
                    z += amount as i64;
                }
                seen.insert(id);
                i += 1;
            }
            Data::FreeDisk(free_space) => {
                let mut new_free_space = free_space;
                // println!("{:?}", files);
                for m in (0..files.len()).rev() {
                    let [key, value] = files[m];
                    let value = value as usize;
                    if value <= new_free_space {
                        new_free_space -= value;
                        update_result(&key, value, &mut z, &mut result);
                        seen.insert(key);
                        files.remove(m);
                    }
                    if free_space == 0 {
                        break;
                    }
                }
                // println!("{:?}", files);
                z += new_free_space as i64;
                i += 1;
            }
        };
    }
    result
}

fn update_result(size: &i64, amount: usize, z: &mut i64, result: &mut i64) {
    for _ in 0..amount {
        // println!("size:{size} z:{z}");
        *result += size * *z;
        *z += 1;
    }
}

fn parse(mut input: &str) -> (Vec<Data>, Vec<[i64; 2]>) {
    input = input.trim();
    let mut result = vec![];
    let mut id = 0;
    let mut files = vec![];
    for (i, char) in input.chars().enumerate() {
        let amount = char.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            result.push(Data::File(id, amount));
            files.push([id, amount as i64]);
            id += 1;
        } else if amount > 0 {
            result.push(Data::FreeDisk(amount));
        }
    }
    (result, files)
}

fn disk_to_str(disk: &Vec<Data>) -> String {
    let mut result = String::from("");
    for data in disk {
        let postfix = match data {
            Data::FreeDisk(amount) => ".".repeat(*amount).to_string(),
            Data::File(id, amount) => id.to_string().repeat(*amount),
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
        assert_eq!(result, 2858);
    }

    #[test]
    fn print() {
        let input = "2333133121414131402";
        let (disk, _) = parse(input);
        let result = disk_to_str(&disk);
        assert_eq!(result, "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn print_2() {
        let input = "12345";
        let (disk, _) = parse(input);
        let result = disk_to_str(&disk);
        assert_eq!(result, "0..111....22222");
    }
}

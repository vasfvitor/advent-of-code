use std::{collections::HashSet, fs};

fn main() {
    _question03_b();
}

fn _question03_b() {
    let file_path = "./src/q03/sample";
    let text = fs::read_to_string(file_path).expect("\n\nWhat?\n\n");
    let mut arr: Vec<(&str, &str)> = Vec::new();
    let text: Vec<&str> = text.split('\n').enumerate().map(|(idx, f)| {
 if idx % 3 = 0 {
    append(f)
    f = 
 }
    }).collect();
    let mut sack: Vec<(&str, &str)> = Vec::new();
    
    println!("{:?}", text);
    /*
    text.iter().for_each(|f| {
        let j = f.trim();
        sack.push(j);
    });
    println!("{:?}", sack);
    let mut a: Vec<u32> = Vec::new();
    sack.iter()
        .for_each(|(f, g, h)| a.append(&mut check_char2(f, g, h)));
    //println!("{:?}", a);
    let _a: u32 = a.iter().sum();
     */
}

fn _question03_a() {
    let file_path = "./src/q03/input";
    let text = fs::read_to_string(file_path).expect("\n\nWhat?\n\n");
    let text: Vec<&str> = text.split('\n').collect();
    let mut sack: Vec<(&str, &str)> = Vec::new();
    text.iter().for_each(|f| {
        let j = f.trim().split_at(f.len() / 2);
        sack.push(j);
    });
    //println!("{:?}", sack);
    let mut a: Vec<u32> = Vec::new();
    sack.iter()
        .for_each(|(f, g)| a.append(&mut check_char(f, g)));

    //println!("{:?}", a);
    let _a: u32 = a.iter().sum();
}
fn check_char2(str1: &str, str2: &str, str3: &str) -> Vec<u32> {
    let set1: HashSet<_> = str1.chars().collect();
    let set2: HashSet<_> = str2.chars().collect();
    let set3: HashSet<_> = str3.chars().collect();

    let common_chars1: HashSet<_> = set1.intersection(&set2).collect();
    let common_chars2: HashSet<_> = set2.intersection(&set3).collect();
    if common_chars1 == common_chars2 {
        let vec: Vec<&char> = common_chars1.into_iter().collect();
        let mut k: Vec<u32> = Vec::new();
        vec.clone().into_iter().for_each(|c| {
            if c.is_uppercase() {
                k.push(*c as u32 - 38);
            } else if c.is_lowercase() {
                k.push(*c as u32 - 96);
            }
        });
        k
    } else {
        let k: Vec<u32> = Vec::new();
        k
    }
}

fn check_char(str1: &str, str2: &str) -> Vec<u32> {
    let set1: HashSet<_> = str1.chars().collect();
    let set2: HashSet<_> = str2.chars().collect();

    let common_chars: HashSet<_> = set1.intersection(&set2).collect();
    let vec: Vec<&char> = common_chars.into_iter().collect();
    let mut k: Vec<u32> = Vec::new();
    vec.clone().into_iter().for_each(|c| {
        if c.is_uppercase() {
            k.push(*c as u32 - 38);
        } else if c.is_lowercase() {
            k.push(*c as u32 - 96);
        }
    });
    k
}

fn _question02_b() {
    /*
    Rock     vs Rock 1 + 3 =     4
    Rock     vs Paper 2 + 6 =    8
    Rock     vs Scissors 3 + 0 = 3
    Paper    vs Rock 1 + 0 =    1
    Paper    vs Paper 2 + 3 =   5
    Paper    vs Scissors 3 + 6 = 9
    Scissors vs Rock 1 + 6 =    7
    Scissors vs Paper 2 + 0 =   2
    Scissors vs Scissors 3 + 3 = 6

    Rock     vs Loss 3     = 3
    Rock     vs Draw 1 + 3 = 4
    Rock     vs Win  2 + 6 = 8
    Paper    vs Loss 1     = 1
    Paper    vs Draw 2 + 3 = 5
    Paper    vs Win  2 + 6 = 9
    Scissors vs Loss 2     = 2
    Scissors vs Draw 3 + 3 = 6
    Scissors vs Win  1 + 6 = 7

     */
    let file_path = "./src/q02/input";
    let text = fs::read_to_string(file_path).expect("\n\nWhat?\n\n");
    let text: Vec<&str> = text.split('\n').collect();
    let mut acc: u32 = 0;
    text.iter().for_each(|t| match t.trim() as &str {
        "A X" => acc += 4,
        "A Y" => acc += 8,
        "A Z" => acc += 3,
        "B X" => acc += 1,
        "B Y" => acc += 5,
        "B Z" => acc += 9,
        "C X" => acc += 7,
        "C Y" => acc += 2,
        "C Z" => acc += 6,
        _ => println!("0"),
    });
    println!("{:?}", acc);
}

fn _question02_a() {
    let file_path = "./src/q02/input";
    let text = fs::read_to_string(file_path).expect("\n\nWhat?\n\n");
    let text: Vec<&str> = text.split('\n').collect();
    let mut acc: u32 = 0;
    text.iter().for_each(|t| match t.trim() as &str {
        "A X" => acc += 3,
        "A Y" => acc += 4,
        "A Z" => acc += 8,
        "B X" => acc += 1,
        "B Y" => acc += 5,
        "B Z" => acc += 9,
        "C X" => acc += 2,
        "C Y" => acc += 6,
        "C Z" => acc += 7,
        _ => println!("0"),
    });
    println!("{:?}", acc);
}

fn _question01_a() {
    let file_path = "./src/q01/input1";
    let text = fs::read_to_string(file_path).expect("\n\nWhat?\n\n");
    //println!("{:?}", text);
    let nums: Vec<i32> = text
        .lines()
        .enumerate()
        .map(|(_, p)| {
            if p == ("\n") {
                0
            } else {
                p.parse::<i32>().unwrap_or(0)
            }
        })
        .collect();
    let (hi, _last) = nums.into_iter().fold((0, 0), |(hi, sum), num| {
        if num == 0 {
            (hi.max(sum), 0)
        } else {
            (hi, sum + num)
        }
    });
    println!("biggest is: {}", hi);

    /*
    let mut sum = 0;
    let mut hi = 0;
    for i in nums {
        if i == 0 {
            if sum > hi {
                hi = sum;
                sum = 0;
            } else {
                sum = 0;
                continue;
            }
        }
        sum += i;
    } */
}

fn _question01_b() {
    let file_path = "./src/q01/input1";
    let text = fs::read_to_string(file_path).expect("\n\nWhat?\n\n");
    //println!("{:?}", text);
    let mut top: Vec<u32> = Vec::new();
    let nums: Vec<u32> = text
        .lines()
        .enumerate()
        .map(|(_, p)| {
            if p == ("\n") {
                0
            } else {
                p.parse::<u32>().unwrap_or(0)
            }
        })
        .collect();
    //println!("{:?}", nums);
    let (_hi, _last) = nums.into_iter().fold((0, 0), |(hi, sum), n| -> (u32, u32) {
        if n == 0 {
            top.push(sum);
            (hi.max(sum), 0)
        } else {
            //println!("hi: {:?}", n);
            (hi, sum + n)
        }
    });
    top.push(_last);
    top.sort();
    top.reverse();
    //println!("top is: {:?}", top);
    top.truncate(3);
    let hi: u32 = top.iter().sum();
    println!("top is: {:?}", top);
    println!("biggest is: {:?}", hi);
}

/*
ttohi not 206361
198041
too low not 197132
 */

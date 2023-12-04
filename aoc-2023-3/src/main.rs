use regex::Regex;

fn has_special_neighbor(metrix: &Vec<Vec<char>>, index: (usize, usize), len: usize) -> bool {
    let re = Regex::new(r"\d|.").unwrap();
    let mut serounding_chars: Vec<char> = Vec::new();
    
    if index.1 != 0 {
        serounding_chars.push(metrix[index.0][index.1-1]);
        serounding_chars.push(metrix[index.0+1][index.1-1]);
        serounding_chars.push(metrix[index.0-1][index.1-1]);
    };
    if index.1 + len != 10 {
        serounding_chars.push(metrix[index.0][index.1+len]);
        serounding_chars.push(metrix[index.0+1][index.1+1]);
        serounding_chars.push(metrix[index.0-1][index.1+1]);
    }

    if index.0 != 0 {
        serounding_chars.append(metrix[index.0-1][index.1..(index.1+len)].to_vec().as_mut());
    }
    if index.0 != metrix.len() {
        serounding_chars.append(metrix[index.0+1][index.1..(index.1+len)].to_vec().as_mut());
    }

    let str: String = serounding_chars.into_iter().collect();
    return re.is_match(str.as_str());
}

fn main() {
    let mut metrix: Vec<Vec<char>> = Vec::new();
    let f = std::fs::read_to_string("./input.txt");
    match f {
        Ok(f) => {
            for line in f.lines().into_iter() {
                metrix.push(line.chars().collect());
                for line in metrix {
                    line.into_iter().collect::<String>().find(r"\d")
                }
            }
        },
        Err(er) => print!("{}", er)
    }
}

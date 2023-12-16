use core::panic;

fn makeStrNumberVec (strs : &str) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let strNumbers = strs.split_whitespace().collect::<Vec<&str>>();
    for number in strNumbers {
        res.push(number.parse().unwrap())
    }
    return res;
}

fn main() {
    let f = std::fs::read_to_string("./data.txt");
    match f {
        Ok(x) => {
            let mut totalScore = 0;
            for (_matchId, line) in x.lines().enumerate() {
                let mut score = 0;
                let (_, numbers) = line.split_once(":").unwrap();

                let (winning, mine ) = numbers.split_once("|").unwrap();
                let mineParsed = makeStrNumberVec(mine);
                let winningParsed = makeStrNumberVec(winning);
                
                for myNumber in mineParsed {
                    if winningParsed.contains(&myNumber) {
                        if score == 0  {
                            score = 1
                        } else {
                            score = score * 2;
                        }
                    }
                }
                totalScore =totalScore + score;
            }
        print!("{}", totalScore);
            return ()
        }
        Err(err) => {
            panic!("{}",err);
    }
}
}

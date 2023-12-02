use std::fs;

#[derive(Debug)]
struct Game {
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl Game {
    fn add_grab(&mut self, grab: Grab) -> () {
        if self.max_blue < grab.blue {self.max_blue = grab.blue}
        if self.max_red < grab.red {self.max_red = grab.red}
        if self.max_green < grab.green {self.max_green = grab.green}
    }
}

#[derive(Debug)]
struct Grab {
    red: u32,
    green: u32,
    blue: u32,
}

impl Grab {
    fn new(hand: &str) -> Grab {
        let mut blue: u32 = 0;
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let colour_pairs: Vec<&str> = hand.split(",").into_iter().collect();
        for mut colour_pair in colour_pairs {
            colour_pair = &colour_pair[1..colour_pair.len()];
            let (count, colour) = colour_pair.split_once(' ').unwrap();
            match colour {
                "red" => red += count.parse::<u32>().unwrap(),
                "green" => green += count.parse::<u32>().unwrap(),
                "blue" => blue += count.parse::<u32>().unwrap(),
                x => print!(
                    "Could not match the colour in the grabl new method:\n text: {}\n",
                    x
                ),
            }
        }
        return Grab {
            red: red,
            green: green,
            blue: blue,
        };
    }

    fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        return !(self.red > max_red || self.green > max_green || self.blue > max_blue);
    }
}

fn main() {

    
    let res = fs::read_to_string("./input.txt");
    match res {
        Ok(contents) => {
            let games: Vec<&str> = contents.lines().into_iter().collect();
                part_two(games)
        }
        Err(e) => panic!("{}", e),
    }
}

fn part_one(games: Vec<&str>) -> () {
    let max_green: u32 = 13;
    let max_red: u32 = 12;
    let max_blue: u32 = 14;
    let mut result: u32 = 0;

    for game in games {
    let mut is_possible = true;
    let (mut id, rest) = game.split_once(":").unwrap();
    (_, id) = id.split_once(" ").unwrap();
    let hands: Vec<&str> = rest.split(";").into_iter().collect();
    for hand in hands {
        let grab = Grab::new(hand);
        if !grab.is_possible(max_red, max_green, max_blue) {
            print!("{:?} makes game {} not possible\n", grab, id);
            is_possible = false;
            break;
        };
    }
    if is_possible {result += id.parse::<u32>().unwrap()};
    }
    print!("{}", result);
}

fn part_two(games: Vec<&str>) -> ()  {
    let mut result: u32 = 0;
    for row in games {
        let mut game = Game{max_blue: 0,max_green: 0, max_red: 0};
        let (_, rest) = row.split_once(":").unwrap();
        let hands: Vec<&str> = rest.split(";").into_iter().collect();
        for hand in hands {
            let grab = Grab::new(hand);
            game.add_grab(grab);
        }
        let power = game.max_blue * game.max_green * game.max_red;
        result += power; 
    }
    print!("{}", result);
}
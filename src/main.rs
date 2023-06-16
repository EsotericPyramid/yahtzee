use rand::Rng;
use std::io;

struct Dice {
    die_one: usize,
    die_two: usize,
    die_three: usize,
    die_four: usize,
    die_five: usize
}

enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    ThreeOfKind,
    FourOfKind,
    FullHouse,
    SmallStraight,
    LargeStraight,
    Yahtzee,
    Chance
}

impl Category {
    fn to_str(&self) -> &str {
        match self {
            Category::Ones => "1: Ones",
            Category::Twos => "2: Twos",
            Category::Threes => "3: Threes",
            Category::Fours => "4: Fours",
            Category::Fives => "5: Fives",
            Category::Sixes => "6: Sixes",
            Category::ThreeOfKind => "7: three of a kind",
            Category::FourOfKind => "8: four of a kind",
            Category::FullHouse => "9: Fullhouse",
            Category::SmallStraight => "10: small straight",
            Category::LargeStraight => "11: large straight",
            Category::Yahtzee => "12: Yahtzee",
            Category::Chance => "13: Chance",
        }
    }
}

impl Dice {
    fn roll(&mut self, roll_die_one: bool, roll_die_two: bool, roll_die_three: bool, roll_die_four: bool, roll_die_five: bool) {
        let mut rng = rand::thread_rng();
        if roll_die_one {
            self.die_one = rng.gen_range(1..=6);
        }
        if roll_die_two {
            self.die_two = rng.gen_range(1..=6);
        }
        if roll_die_three {
            self.die_three = rng.gen_range(1..=6);
        }
        if roll_die_four {
            self.die_four = rng.gen_range(1..=6);
        }
        if roll_die_five {
            self.die_five = rng.gen_range(1..=6);
        }
    }

    fn score_category(&self, cat: &Category) -> usize {
        match cat {
            Category::Ones => {
                let mut total: usize = 0;
                if self.die_one == 1 {
                    total += 1;
                }
                if self.die_two == 1 {
                    total += 1;
                }
                if self.die_three == 1 {
                    total += 1;
                }
                if self.die_four == 1 {
                    total += 1;
                }
                if self.die_five == 1 {
                    total += 1;
                }
                total
            }
            Category::Twos => {
                let mut total: usize = 0;
                if self.die_one == 2 {
                    total += 1;
                }
                if self.die_two == 2 {
                    total += 1;
                }
                if self.die_three == 2 {
                    total += 1;
                }
                if self.die_four == 2 {
                    total += 1;
                }
                if self.die_five == 2 {
                    total += 1;
                }
                2*total
            }
            Category::Threes => {
                let mut total: usize = 0;
                if self.die_one == 3 {
                    total += 1;
                }
                if self.die_two == 3 {
                    total += 1;
                }
                if self.die_three == 3 {
                    total += 1;
                }
                if self.die_four == 3 {
                    total += 1;
                }
                if self.die_five == 3 {
                    total += 1;
                }
                3*total
            }
            Category::Fours => {
                let mut total: usize = 0;
                if self.die_one == 4 {
                    total += 1;
                }
                if self.die_two == 4 {
                    total += 1;
                }
                if self.die_three == 4 {
                    total += 1;
                }
                if self.die_four == 4 {
                    total += 1;
                }
                if self.die_five == 4 {
                    total += 1;
                }
                4*total
            }
            Category::Fives => {
                let mut total: usize = 0;
                if self.die_one == 5 {
                    total += 1;
                }
                if self.die_two == 5 {
                    total += 1;
                }
                if self.die_three == 5 {
                    total += 1;
                }
                if self.die_four == 5 {
                    total += 1;
                }
                if self.die_five == 5 {
                    total += 1;
                }
                5*total
            }
            Category::Sixes => {
                let mut total: usize = 0;
                if self.die_one == 6 {
                    total += 1;
                }
                if self.die_two == 6 {
                    total += 1;
                }
                if self.die_three == 6 {
                    total += 1;
                }
                if self.die_four == 6 {
                    total += 1;
                }
                if self.die_five == 6 {
                    total += 1;
                }
                6*total
            }
            Category::ThreeOfKind => {
                self.die_one+self.die_two+self.die_three+self.die_four+self.die_five
            }
            Category::FourOfKind => {
                self.die_one+self.die_two+self.die_three+self.die_four+self.die_five
            }
            Category::FullHouse => 25,
            Category::SmallStraight => 30,
            Category::LargeStraight => 40,
            Category::Yahtzee => 50,
            Category::Chance => {
                self.die_one+self.die_two+self.die_three+self.die_four+self.die_five
            }
        }
    }

    fn is_valid_category(&self, cat: &Category) -> bool {
        match cat {
            Category::Ones => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                f_array[0] >= 1
            }
            Category::Twos => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                f_array[1] >= 1
            }
            Category::Threes => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                f_array[2] >= 1
            }
            Category::Fours => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                f_array[3] >= 1
            }
            Category::Fives => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                f_array[4] >= 1
            }
            Category::Sixes => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                f_array[5] >= 1
            }
            Category::ThreeOfKind => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                let mut output = false;
                for val in f_array {
                    if val == 3 {
                        output = true
                    }
                }
                output
            }
            Category::FourOfKind => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                let mut output = false;
                for val in f_array {
                    if val == 4 {
                        output = true
                    }
                }
                output
            }
            Category::FullHouse =>  {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                let mut output_one = false;
                let mut output_two: bool = false;
                for val in f_array {
                    if val == 3 {
                        output_one = true
                    }
                    if val == 2 {
                        output_two = true
                    }
                }
                output_one && output_two
            }
            Category::SmallStraight => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                let mut output = false;
                if f_array[2] >= 1 && f_array[3] >= 1 {
                    if f_array[1] >= 1 {
                        if f_array[0] >= 1 {
                            output = true
                        }
                        else if f_array[4] >= 1 {
                            output = true
                        } 
                    } else if f_array[4] >= 1 && f_array[5] >= 1 {
                        output = true
                    }
                }
                output
            }
            Category::LargeStraight => {
                let mut f_array = [0; 6];
                let die_array = [self.die_one,self.die_two,self.die_three,self.die_four,self.die_five];
                for die in die_array.iter() {
                    f_array[die-1] = f_array[die-1] +1
                }
                let mut output = false;
                if f_array[1] >= 1 && f_array[2] >= 1 && f_array[3] >= 1 && f_array[4] >= 1 {
                    if f_array[0] >= 1 {
                        output = true
                    } 
                    else if  f_array[5] >= 1  {
                        output = true
                    }
                }
                output
            }
            Category::Yahtzee => {
                self.die_one == self.die_two && self.die_one == self.die_three && self.die_one == self.die_four && self.die_one == self.die_five
            }
            Category::Chance => true
        }
    }

    fn to_str(&self) -> String {
        format!("{}, {}, {}, {}, {}",self.die_one,self.die_two,self.die_three,self.die_four,self.die_five)
    }
}

fn main() {
    let mut score = 0;
    let mut d = Dice{
        die_one: 0,
        die_two: 0,
        die_three: 0,
        die_four: 0,
        die_five: 0
    };
    let categories: [Category; 13] = [
        Category::Ones,
        Category::Twos,
        Category::Threes,
        Category::Fours,
        Category::Fives,
        Category::Sixes,
        Category::ThreeOfKind,
        Category::FourOfKind,
        Category::FullHouse,
        Category::SmallStraight,
        Category::LargeStraight,
        Category::Yahtzee,
        Category::Chance];
    let mut used_categories: [bool; 13] = [false; 13];
    let mut guess: usize;
    let mut buffer: String = String::new();
    let mut turn_count = 1;
    let input_handler = io::stdin();
    let mut rerolled_dice: [bool; 5] = [true; 5];
    let mut no_cat_flag: bool;
    while turn_count <= 13 {
        println!("Remaining Categories");
        for cat_num in 0..13 {
            if ! used_categories[cat_num] {
                println!("{}",categories[cat_num].to_str());
            }
        }
        d.roll(true,true,true,true,true);
        println!("\n{}\n",d.to_str());
        for _test in 1..3 {
            for die in 0..5 {
                input_handler.read_line(&mut buffer).expect("Failed to read line (338)");
                if buffer == "y\n" {
                    rerolled_dice[die] = true;
                } else {
                    rerolled_dice[die] = false;
                }
                buffer = String::new();
            }
            d.roll(rerolled_dice[0],rerolled_dice[1],rerolled_dice[2],rerolled_dice[3],rerolled_dice[4]);
            println!("\n{}\n",d.to_str());
        }
        no_cat_flag = true;
        for cat_num in 0..13 {
            if d.is_valid_category(&categories[cat_num]) {
                if ! used_categories[cat_num] {
                    println!("{}", categories[cat_num].to_str());
                    no_cat_flag = false;
                }
            }
        }
        if no_cat_flag {
            println!("No valid category :(");
            println!("\n{}\n",score);
            turn_count += 1;
        }
        println!("");
        input_handler.read_line(&mut buffer).expect("Failed to read line (342)");
        guess = buffer.trim().parse().expect("Please type a number!");
        buffer = String::new();
        if used_categories[guess-1] {
            panic!("not a valid Category")
        } else {
            used_categories[guess-1] = true
        }
        score = score + d.score_category(&categories[guess-1]);
        println!("\n{}\n",score);
        turn_count += 1;
    } 
}
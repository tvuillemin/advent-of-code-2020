#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::Read;

lazy_static! {
    static ref EYE_COLOR_RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref HEIGHT_RE: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
}

fn year_is_between(year_str: &str, min: u32, max: u32) -> bool {
    let year = year_str.parse::<u32>().unwrap();
    (min <= year) && (year <= max)
}

fn check_height(height: &str) -> bool {
    if HEIGHT_RE.is_match(height) {
        let captures = HEIGHT_RE.captures(height).unwrap();
        let value = captures[1].parse::<u32>().unwrap();
        let unit = &captures[2];

        return (unit == "cm") && (150 <= value) && (value <= 193)
            || (unit == "in") && (59 <= value) && (value <= 76);
    }
    false
}

struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new(line_group: &Vec<&str>) -> Passport {
        let mut passport = Passport {
            birth_year: None,
            country_id: None,
            expiration_year: None,
            eye_color: None,
            hair_color: None,
            height: None,
            issue_year: None,
            passport_id: None,
        };
        for token in line_group.iter() {
            let item: Vec<&str> = token.split(':').collect();
            let key = item[0];
            let value = item[1];
            if key == "byr" {
                passport.birth_year = Some(String::from(value));
            } else if key == "cid" {
                passport.country_id = Some(String::from(value));
            } else if key == "eyr" {
                passport.expiration_year = Some(String::from(value));
            } else if key == "ecl" {
                passport.eye_color = Some(String::from(value));
            } else if key == "hcl" {
                passport.hair_color = Some(String::from(value));
            } else if key == "hgt" {
                passport.height = Some(String::from(value));
            } else if key == "iyr" {
                passport.issue_year = Some(String::from(value));
            } else if key == "pid" {
                passport.passport_id = Some(String::from(value));
            }
        }
        passport
    }

    fn is_valid(&self) -> bool {
        self.birth_year_is_valid()
            && self.expiration_year_is_valid()
            && self.eye_color_is_valid()
            && self.hair_color_is_valid()
            && self.height_is_valid()
            && self.issue_year_is_valid()
            && self.passport_id_is_valid()
    }

    fn birth_year_is_valid(&self) -> bool {
        match &self.birth_year {
            Some(year) => year_is_between(year, 1920, 2002),
            None => false,
        }
    }

    fn expiration_year_is_valid(&self) -> bool {
        match &self.expiration_year {
            Some(year) => year_is_between(year, 2020, 2030),
            None => false,
        }
    }

    fn eye_color_is_valid(&self) -> bool {
        match &self.eye_color {
            Some(eye_color) => EYE_COLOR_RE.is_match(eye_color),
            None => false,
        }
    }

    fn hair_color_is_valid(&self) -> bool {
        match &self.hair_color {
            Some(hair_color) => HAIR_COLOR_RE.is_match(hair_color),
            None => false,
        }
    }

    fn height_is_valid(&self) -> bool {
        match &self.height {
            Some(height) => check_height(height),
            None => false,
        }
    }

    fn issue_year_is_valid(&self) -> bool {
        match &self.issue_year {
            Some(year) => year_is_between(year, 2010, 2020),
            None => false,
        }
    }

    fn passport_id_is_valid(&self) -> bool {
        match &self.passport_id {
            Some(passport_id) => PASSPORT_ID_RE.is_match(passport_id),
            None => false,
        }
    }
}

fn get_lines() -> Vec<String> {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input_data = String::new();
    input_file.read_to_string(&mut input_data).unwrap();

    input_data
        .split('\n')
        .map(|line| String::from(line))
        .collect()
}

fn main() {
    let mut valid_passport_nb = 0;
    let mut line_group: Vec<&str> = Vec::new();

    for line in get_lines().iter() {
        if line == "" {
            let passport = Passport::new(&line_group);
            if passport.is_valid() {
                valid_passport_nb += 1;
            }
            line_group.clear()
        } else {
            line_group.extend(line.split(" "))
        }
    }

    println!("{}", valid_passport_nb);
}

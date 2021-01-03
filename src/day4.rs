extern crate regex;
use regex::Regex;
use std::error::Error;

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
  fn is_valid(&self) -> bool {
    if self.birth_year.is_some()
      && self.issue_year.is_some()
      && self.expiration_year.is_some()
      && self.height.is_some()
      && self.hair_color.is_some()
      && self.eye_color.is_some()
      && self.passport_id.is_some()
    {
      true
    } else {
      false
    }
  }

  fn is_really_valid(&self) -> bool {
    if !self.is_valid() {
      return false;
    }

    self.validate_birth()
      && self.validate_hair()
      && self.validate_issue()
      && self.validate_expiry()
      && self.validate_height()
      && self.validate_eye()
      && self.validate_pid()
  }

  fn validate_birth(&self) -> bool {
    validate_number(self.birth_year.clone(), 1920, 2002)
  }

  fn validate_issue(&self) -> bool {
    validate_number(self.issue_year.clone(), 2010, 2020)
  }

  fn validate_expiry(&self) -> bool {
    validate_number(self.expiration_year.clone(), 2020, 2030)
  }

  fn validate_height(&self) -> bool {
    validate_height(self.height.clone())
  }

  fn validate_hair(&self) -> bool {
    validate_hair(self.hair_color.clone())
  }

  fn validate_eye(&self) -> bool {
    validate_eye(self.eye_color.clone())
  }

  fn validate_pid(&self) -> bool {
    validate_pid(self.passport_id.clone())
  }
}

fn validate_eye(val: Option<String>) -> bool {
  let valid = vec!["brn", "amb", "blu", "gry", "grn", "hzl", "oth"];
  if let Some(val) = val {
    return valid.contains(&val.as_str());
  }
  false
}

fn validate_hair(val: Option<String>) -> bool {
  let re = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
  if let Some(val) = val {
    return re.is_match(&val);
  }
  false
}

fn validate_height(val: Option<String>) -> bool {
  let re = Regex::new(r"(\d+)(in|cm)").unwrap();
  if let Some(val) = val {
    if !re.is_match(&val) {
      return false;
    }
    let captures = re.captures(&val).unwrap();
    let height = captures
      .get(1)
      .map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let suffix = captures.get(2).map_or("", |m| m.as_str());
    let result: bool = match suffix {
      "in" => height >= 59 && height <= 76,
      "cm" => height >= 150 && height <= 193,
      _ => false,
    };
    return result;
  }
  false
}

fn validate_number(val: Option<String>, low: i64, high: i64) -> bool {
  if let Some(val) = val {
    if val.len() != 4 {
      return false;
    }
    let val = val.parse().unwrap_or(0);
    if val < low || val > high {
      return false;
    }
    return true;
  }
  false
}

fn validate_pid(val: Option<String>) -> bool {
  let re = Regex::new(r"^\d{9}$").unwrap();
  if let Some(val) = val {
    return re.is_match(&val);
  }
  false
}

fn parse_input(input: &str) -> Vec<Passport> {
  input.split("\n\n").map(|c| parse_card(&c)).collect()
}

fn parse_card(card: &str) -> Passport {
  let items: Vec<&str> = card.split(|x| x == ' ' || x == '\n').collect();
  let mut passport = Passport {
    birth_year: None,
    issue_year: None,
    expiration_year: None,
    height: None,
    hair_color: None,
    eye_color: None,
    passport_id: None,
    country_id: None,
  };

  for item in items {
    let parsed = item.split(':').take(2).collect::<Vec<&str>>();
    if let [key, val] = parsed[..] {
      let val = val.to_string();
      match key {
        "byr" => passport.birth_year = Some(val),
        "iyr" => passport.issue_year = Some(val),
        "eyr" => passport.expiration_year = Some(val),
        "hgt" => passport.height = Some(val),
        "hcl" => passport.hair_color = Some(val),
        "ecl" => passport.eye_color = Some(val),
        "pid" => passport.passport_id = Some(val),
        "cid" => passport.country_id = Some(val),
        other => panic!("Unexpected key: {}", other),
      }
    }
  }

  passport
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_input(input).iter().filter(|x| x.is_valid()).count() as i64)
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(
    parse_input(input)
      .iter()
      .filter(|x| x.is_really_valid())
      .count() as i64,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_height() {
    let valid = vec![Some(String::from("60in")), Some(String::from("190cm"))];
    let invalid = vec![None, Some(String::from("190in")), Some(String::from("190"))];

    for val in valid {
      assert_eq!(true, validate_height(val))
    }

    for val in invalid {
      assert_eq!(false, validate_height(val));
    }
  }

  #[test]
  fn test_hair() {
    let valid = vec![Some(String::from("#123abc"))];
    let invalid = vec![
      None,
      Some(String::from("123abc")),
      Some(String::from("#123abc3")),
      Some(String::from("#123abz")),
    ];
    for val in valid {
      assert_eq!(true, validate_hair(val));
    }

    for val in invalid {
      assert_eq!(false, validate_hair(val));
    }
  }

  #[test]
  fn test_number() {
    let valid = vec![
      Some(String::from("2002")),
      Some(String::from("1920")),
      Some(String::from("1990")),
    ];
    let invalid = vec![
      None,
      Some(String::from("2003")),
      Some(String::from("1919")),
      Some(String::from("004")),
    ];

    for val in valid {
      assert_eq!(true, validate_number(val, 1920, 2002));
    }

    for val in invalid {
      assert_eq!(false, validate_number(val, 1920, 2002));
    }
  }

  #[test]
  fn test_eye() {
    let valid = vec!["brn", "amb", "blu", "gry", "grn", "hzl", "oth"];
    let invalid = vec![
      None,
      Some(String::from("bad")),
      Some(String::from("")),
      Some(String::from("gr")),
      Some(String::from("wat")),
    ];

    for val in valid {
      assert_eq!(true, validate_eye(Some(String::from(val))));
    }

    for val in invalid {
      assert_eq!(false, validate_eye(val));
    }
  }

  #[test]
  fn test_pid() {
    let valid = vec!["000000001", "123456789"];
    let invalid = vec![
      None,
      Some(String::from("00000000")),
      Some(String::from("00000000765")),
      Some(String::from("00000000x")),
    ];

    for val in valid {
      assert_eq!(true, validate_pid(Some(String::from(val))));
    }

    for val in invalid {
      assert_eq!(false, validate_pid(val));
    }
  }
}

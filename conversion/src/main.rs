use std::{convert::{From, Into, TryFrom}, fmt::Display, num::ParseIntError, str::FromStr};

#[allow(warnings)]
#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug)]
struct AnotherNumber {
    value: i32,
}

impl Display for AnotherNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value for another_number: {}", self.value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value: value }
    }
}

// dummy experiment
impl From<&str> for Number {
    fn from(value: &str) -> Self {
        if value == "one" {
            Number {value: 1}
        } else {
            Number {value: 0}
        }
    }
}

impl Into<AnotherNumber> for i32 {
    fn into(self) -> AnotherNumber {
        AnotherNumber { value: self }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value %2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    from();
    into();
    dummy_experiment();
    try_from_and_try_into();
    let result = dummy_result_experiment(-1);
    match result {
        Ok(positive) => println!("got: {}", positive),
        Err(negative) => println!("got: {}", negative),
    };
    test_with_FromStr();
    to_and_from_strings();
}

fn from() {
    let num = Number::from(30);
    println!("my number is {:?}", num);
}

fn into() {
    let int = 5;
    let num: AnotherNumber = int.into();
    println!("my number is {:?}", num);
}

fn dummy_experiment() {
    let one = Number::from("one");
    println!("value is {:?}", one);

    let not_one = Number::from("***");
    println!("value is {:?}", not_one);
}

fn try_from_and_try_into() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

fn dummy_result_experiment(value: i32) -> Result<String, AnotherNumber> {
    if value >= 0 {
        Ok(String::from("positive! ok"))
    } else {
        Err(AnotherNumber { value: value })
    }
}

#[allow(warnings)]
fn test_with_FromStr() {

    #[derive(Debug, PartialEq)]
    struct SemicolonSeparatedValue {
        values: Vec<String>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ParseError;

    impl FromStr for SemicolonSeparatedValue {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parsed: Vec<String> = s
                .split(";")
                .map(|s| String::from(s))
                .collect();

            if parsed.len() > 0 {
                Ok(SemicolonSeparatedValue { values: parsed })
            } else {
                Err(ParseError)
            }
        }
    }

    let parsed = SemicolonSeparatedValue::from_str("a;b;c");
    match parsed {
        Ok(value) => println!("{:?}", value),
        Err(error) => println!("{:?}", error),
    }
}

fn to_and_from_strings() {
    #[derive(Debug)]
    struct Circle {
        radius: i32
    }

    impl Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "circle of radius: {}", self.radius)
        }
    }

    impl FromStr for Circle {
        type Err = ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().parse() {
                Ok(num) => Ok(Circle { radius: num }),
                Err(e) => Err(e),
            }
        }
    }

    let circle = Circle {radius: 6};
    println!("{}", circle);

    //--------

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("{}", sum);

    let radius = "  3 ";
    let circle: Circle = radius.parse().unwrap();
    println!("{}", circle);
}


use std::fmt::Display;
use std::io::BufRead;
use std::ops::Add;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Regular(u64),
}

fn main() -> Result<()> {
    let input = read_input()?;

    let sum = input.iter().cloned().reduce(|acc, x| acc + x).unwrap();
    println!(
        "sum of entire input = {}; magnitude = {}",
        sum,
        sum.magnitude()
    );

    let max = input
        .iter()
        .flat_map(|a| input.iter().map(|b| (a.clone() + b.clone()).magnitude()))
        .max()
        .unwrap();

    println!("maximum magnitude of summing 2 numbers = {}", max);

    Ok(())
}

fn read_input() -> Result<Vec<SnailfishNumber>> {
    let stdin = std::io::stdin();
    let mut nums = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        let number = SnailfishNumber::parse(&mut line.as_str());
        nums.push(number);
    }
    Ok(nums)
}

impl SnailfishNumber {
    fn pair(left: Self, right: Self) -> Self {
        Self::Pair(Box::new(left), Box::new(right))
    }

    fn parse(input: &mut &str) -> Self {
        let chomp_char = |expected_ch: char, input: &mut &str| match input.chars().next() {
            Some(c) if c == expected_ch => {
                *input = &input[1..];
            }
            c => panic!("expected a '{}'; got {:?}", expected_ch, c),
        };
        let chomp_digits = |input: &mut &str| -> u64 {
            if let Some(split_idx) = input.chars().position(|c| !c.is_digit(10)) {
                let (digits_str, rest) = input.split_at(split_idx);
                *input = rest;
                digits_str.parse().unwrap()
            } else {
                panic!("expected digits to end")
            }
        };
        match input.chars().next() {
            Some(c) if c.is_digit(10) => {
                return Self::Regular(chomp_digits(input));
            }
            Some('[') => {
                chomp_char('[', input);
                let left = Self::parse(input);
                chomp_char(',', input);
                *input = input.trim_start();
                let right = Self::parse(input);
                chomp_char(']', input);
                return Self::pair(left, right);
            }
            c => panic!("expected snailfish number; got {:?}", c),
        }
    }

    fn add_to_rightmost(&mut self, n: u64) -> bool {
        match self {
            Self::Regular(value) => {
                *value += n;
                return true;
            }
            Self::Pair(left, right) => {
                if right.add_to_rightmost(n) {
                    return true;
                }
                return left.add_to_rightmost(n);
            }
        }
    }

    fn add_to_leftmost(&mut self, n: u64) -> bool {
        match self {
            Self::Regular(value) => {
                *value += n;
                return true;
            }
            Self::Pair(left, right) => {
                if left.add_to_leftmost(n) {
                    return true;
                }
                return right.add_to_leftmost(n);
            }
        }
    }

    fn explode(&mut self, depth: u32) -> Option<(Option<u64>, Option<u64>)> {
        match self {
            Self::Pair(left, right) => {
                if depth == 4 {
                    let (&left, &right) = match (left.as_ref(), right.as_ref()) {
                        (Self::Regular(left), Self::Regular(right)) => (left, right),
                        o => panic!("expected a pair of regular numbers at depth 4; got {:?}", o),
                    };
                    *self = Self::Regular(0);
                    Some((Some(left), Some(right)))
                } else {
                    if let Some((left_result, right_result)) = left.explode(depth + 1) {
                        if let Some(child_right) = right_result {
                            if right.add_to_leftmost(child_right) {
                                return Some((left_result, None));
                            }
                        }
                        return Some((left_result, right_result));
                    }

                    if let Some((left_result, right_result)) = right.explode(depth + 1) {
                        if let Some(child_left) = left_result {
                            if left.add_to_rightmost(child_left) {
                                return Some((None, right_result));
                            }
                            return Some((left_result, right_result));
                        }
                        return Some((left_result, right_result));
                    }

                    None
                }
            }
            _ => None,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            &mut Self::Regular(n) if n >= 10 => {
                let div2 = n / 2;
                *self = Self::pair(Self::Regular(div2), Self::Regular(n - div2));
                true
            }
            Self::Regular(_) => false,
            Self::Pair(left, right) => left.split() || right.split(),
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            return self;
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            &Self::Regular(n) => n,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Regular(n) => write!(formatter, "{}", n),
            Self::Pair(left, right) => write!(formatter, "[{},{}]", left, right),
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        SnailfishNumber::pair(self, other).reduce()
    }
}

#[cfg(test)]
mod test {
    use super::SnailfishNumber as SN;
    use SN::Regular as R;

    #[test]
    fn simple_left_explode() {
        let mut n = SN::pair(
            R(0),
            SN::pair(R(1), SN::pair(R(2), SN::pair(R(3), SN::pair(R(4), R(5))))),
        );
        let result = n.explode(0);
        assert_eq!(result, Some((None, Some(5))));
        assert_eq!(
            n,
            SN::pair(R(0), SN::pair(R(1), SN::pair(R(2), SN::pair(R(7), R(0)))))
        );
    }

    #[test]
    fn simple_right_explode() {
        let mut n = SN::pair(
            SN::pair(SN::pair(SN::pair(SN::pair(R(1), R(2)), R(3)), R(4)), R(5)),
            R(6),
        );
        let result = n.explode(0);
        assert_eq!(result, Some((Some(1), None)));
        assert_eq!(
            n,
            SN::pair(SN::pair(SN::pair(SN::pair(R(0), R(5)), R(4)), R(5)), R(6))
        );
    }

    macro_rules! explode_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let mut parsed = SN::parse(&mut input.as_ref());
                assert!(parsed.explode(0).is_some());
                assert_eq!(format!("{}", parsed), expected);
            }
        )*
        }
    }

    explode_tests! {
        example_1: ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        example_2: ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        example_3: ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        example_4: (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        example_5: (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
    }

    #[test]
    fn reduce_test() {
        let input = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let parsed = SN::parse(&mut input.as_ref());
        assert_eq!(
            format!("{}", parsed.reduce()),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }
}

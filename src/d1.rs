use crate::{read_file, PuzzleRun};
use lazy_static::lazy_static;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

#[allow(dead_code)]
struct Part1;

impl Part1 {
    fn _extract(line: &str) -> anyhow::Result<u32> {
        Ok(10
            * line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap()
            + line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap())
    }
}

struct Part2;

enum CheckResult {
    Nan,
    Number(u8),
    Candidate,
}

lazy_static! {
    static ref DIGITS: [Vec<u8>; 10] = [
        b"zero".to_vec(),
        b"one".to_vec(),
        b"two".to_vec(),
        b"three".to_vec(),
        b"four".to_vec(),
        b"five".to_vec(),
        b"six".to_vec(),
        b"seven".to_vec(),
        b"eight".to_vec(),
        b"nine".to_vec(),
    ];
}

impl Part2 {
    fn check_forward(buf: &[u8]) -> CheckResult {
        for i in 0..DIGITS.len() {
            if buf == DIGITS[i] {
                return CheckResult::Number(i as u8);
            }
            if buf.len() < DIGITS[i].len() && buf[..] == DIGITS[i].as_slice()[0..buf.len()] {
                return CheckResult::Candidate;
            }
        }
        CheckResult::Nan
    }

    fn check_backward(buf: &[u8]) -> CheckResult {
        for i in 0..DIGITS.len() {
            if buf.len() >= DIGITS[i].len() && buf[0..DIGITS[i].len()] == DIGITS[i] {
                return CheckResult::Number(i as u8);
            }
        }
        CheckResult::Nan
    }

    fn extract(line: &str) -> u32 {
        assert!(line.is_ascii());
        let bytes = line.as_bytes();

        let fw = {
            let mut buf: Vec<u8> = Vec::new();
            let mut i = 0;
            let mut val: Option<u8> = None;
            while i < bytes.len() {
                if bytes[i].is_ascii_digit() {
                    val = Some(bytes[i] - 48);
                    break;
                }
                buf.push(bytes[i]);
                match Self::check_forward(&buf) {
                    CheckResult::Number(n) => {
                        val = Some(n);
                        break;
                    }
                    CheckResult::Nan => {
                        buf.remove(0);
                    }
                    CheckResult::Candidate => {}
                }
                i += 1;
            }
            val.unwrap()
        };

        let bw = {
            let mut i = bytes.len() - 1;
            let val: Option<u8>;
            loop {
                if bytes[i].is_ascii_digit() {
                    val = Some(bytes[i] - 48);
                    break;
                }
                match Self::check_backward(&bytes[i..]) {
                    CheckResult::Number(n) => {
                        val = Some(n);
                        break;
                    }
                    CheckResult::Nan => {}
                    CheckResult::Candidate => {}
                }
                if i == 0 {
                    panic!();
                } else {
                    i -= 1;
                }
            }
            val.unwrap()
        };
        println!("{}: {}", line, fw as u32 * 10 + bw as u32);
        fw as u32 * 10 + bw as u32
    }
}
impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day1.txt")
    }

    fn run(&self, input: &str) -> String {
        let sum: u32 = input.lines().map(Part2::extract).sum();
        format!("{}", sum)
    }
}

#[cfg(test)]
mod test {

    use super::CheckResult;
    use super::Part2;

    #[test]
    fn test_check_forward() {
        let buf = b"six";
        assert!(matches!(
            Part2::check_forward(buf.as_slice()),
            CheckResult::Number(6)
        ));

        let buf = b"si";
        assert!(matches!(
            Part2::check_forward(buf.as_slice()),
            CheckResult::Candidate
        ));

        let buf = b"q";
        assert!(matches!(
            Part2::check_forward(buf.as_slice()),
            CheckResult::Nan
        ));

        let buf = b"siq";
        assert!(matches!(
            Part2::check_forward(buf.as_slice()),
            CheckResult::Nan
        ));
    }

    #[test]
    fn test_check_backward() {
        let buf = b"six";
        assert!(matches!(
            Part2::check_backward(buf.as_slice()),
            CheckResult::Number(6)
        ));

        let buf = b"ix";
        assert!(matches!(
            Part2::check_backward(buf.as_slice()),
            CheckResult::Nan
        ));

        let buf = b"qix";
        assert!(matches!(
            Part2::check_backward(buf.as_slice()),
            CheckResult::Nan
        ));

        let buf = b"threexxx";
        assert!(matches!(
            Part2::check_backward(buf.as_slice()),
            CheckResult::Number(3)
        ));
    }

    #[test]
    fn test_extract() {
        let buf = "two1nine";
        assert_eq!(Part2::extract(buf), 29);

        let buf = "eightwothree";
        assert_eq!(Part2::extract(buf), 83);

        let buf = "abcone2threexyz";
        assert_eq!(Part2::extract(buf), 13);

        let buf = "xtwone3four";
        assert_eq!(Part2::extract(buf), 24);

        let buf = "4nineeightseven2";
        assert_eq!(Part2::extract(buf), 42);

        let buf = "zoneight234";
        assert_eq!(Part2::extract(buf), 14);

        let buf = "7pqrstsixteen";
        assert_eq!(Part2::extract(buf), 76);

        let buf = "eighthree";
        assert_eq!(Part2::extract(buf), 83);

        let buf = "sevenine";
        assert_eq!(Part2::extract(buf), 79);

        let buf = "5hgkfhkqvbj";
        assert_eq!(Part2::extract(buf), 55);

        let buf = "slqhhldzjfdjzeightlskpbpcd5";
        assert_eq!(Part2::extract(buf), 85);
    }
}

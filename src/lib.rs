#[derive(Debug)]
enum State {
    Start,
    Equal,
    Lesser,
    Higher,
}

#[derive(Debug)]
pub struct BinarySearch<T> {
    lo: usize,
    hi: usize,
    mid: usize,
    pub data: Vec<T>,
    state: State,
}

impl<T> BinarySearch<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + std::fmt::Debug,
{
    pub fn new(input: Vec<T>) -> Self {
        let lo: usize = 0;
        let hi: usize = input.len();
        let mid = (((lo + hi) / 2) as f64).floor() as usize;
        let mut data = input;
        data.sort();
        let state = State::Start;
        Self {
            lo,
            hi,
            mid,
            data,
            state,
        }
    }

    fn reassign_mid_downward(&self) -> usize {
        (((self.lo + self.hi) / 2) as f64).floor() as usize
    }

    fn reassign_mid_upward(&self) -> usize {
        self.lo + (((self.hi - self.lo) / 2) as f64).ceil() as usize
    }

    pub fn find(&mut self, seek: &T) -> Option<usize> {
        let mut value_of_mid = self.data.get(self.mid).unwrap(); // it's safe to unwrap here

        loop {
            match self.state {
                State::Start => {
                    if value_of_mid == seek {
                        self.state = State::Equal;
                        return Some(self.mid);
                    } else if value_of_mid > seek {
                        self.state = State::Higher;
                    } else if value_of_mid < seek {
                        self.state = State::Lesser;
                    }
                }
                State::Higher => {
                    self.hi = self.mid;
                    self.mid = self.reassign_mid_downward();
                    value_of_mid = self.data.get(self.mid).unwrap();

                    if value_of_mid == seek {
                        self.state = State::Equal;
                        return Some(self.mid);
                    } else if value_of_mid < seek {
                        self.state = State::Lesser;
                    }
                }
                State::Lesser => {
                    self.lo = self.mid;
                    self.mid = self.reassign_mid_upward();
                    value_of_mid = self.data.get(self.mid).unwrap();

                    if value_of_mid == seek {
                        self.state = State::Equal;
                        return Some(self.mid);
                    } else if value_of_mid > seek {
                        self.state = State::Higher;
                    }
                }
                State::Equal => return Some(self.mid),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const MIN_REPS: usize = 0;
    const MAX_REPS: usize = 100_000_000;

    fn create_input() -> Vec<usize> {
        let mut input = Vec::<usize>::new();
        let reps = MIN_REPS..=MAX_REPS;
        let mut num = 0;
        for _ in reps {
            input.push(num);
            num += 1;
        }
        input
    }

    #[test]
    fn my_binary_search() {
        let input = create_input();
        let mut binary_search = BinarySearch::new(input);
        let index = binary_search.find(&69_696_969);
        assert_eq!(index, Some(69_696_969));
    }

    #[test]
    fn std_lib_binary_search() {
        let mut input = create_input();
        input.sort();
        let index = input.binary_search(&69_696_969);
        assert_eq!(index, Ok(69_696_969));
    }

    #[test]
    fn test3() {
        let mut input = create_input();
        input.sort();
        let index = input.iter().find(|x| **x == 69_696_969);
        assert_eq!(index, Some(&69_696_969));
    }

    #[test]
    fn test4() {
        let mut input = create_input();
        input.sort();
        let mut iter = input.iter();
        let mut index = -1;
        while let Some(num) = iter.next() {
            index += 1;
            if *num == 69_696_969 {
                break;
            }
        }
        assert_eq!(index, 69_696_969);
    }
}

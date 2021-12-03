use std::fs;

fn get_instructions() -> String {
    let filename = "data/day_1.txt";

    fs::read_to_string(filename).unwrap()
}

pub fn solve_part1() -> i32 {
    let s = get_instructions();

    find_floor(&s)
}

pub fn solve_part2() -> u32 {
    let s = get_instructions();

    find_basement_instruction(&s)
}

pub fn find_floor(s: &str) -> i32 {
    s.chars().fold(0, |acc, x| if x == '(' { acc + 1 } else if x == ')' { acc - 1 } else { acc })
}

pub fn find_basement_instruction(s: &str) -> u32 {
    let mut count = 0;
    let mut floor = 0;

    for c in s.chars() {
        count += 1;
        if c == '(' {
            floor += 1;
        } else if c == ')'{
            floor -= 1;
        }
        if floor < 0 {
            return count
        }
    } count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_0() {
        assert_eq!(find_floor("(())"), 0);
        assert_eq!(find_floor("()()"), 0);
    }

    #[test]
    fn floor_3() {
        assert_eq!(find_floor("((("), 3);
        assert_eq!(find_floor("(()(()("), 3);
        assert_eq!(find_floor("))((((("), 3);
    }

    #[test]
    fn floor_min1() {
        assert_eq!(find_floor("())"), -1);
        assert_eq!(find_floor("))("), -1);
    }

    #[test]
    fn floor_min3() {
        assert_eq!(find_floor(")))"), -3);
        assert_eq!(find_floor(")())())"), -3);
    }

    #[test]
    fn in_basement_at1() {
        assert_eq!(find_basement_instruction(")"), 1);
    }

    #[test]
    fn in_basement_at5() {
        assert_eq!(find_basement_instruction("()())"), 5);
    }
}

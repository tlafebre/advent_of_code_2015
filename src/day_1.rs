pub fn find_floor(s: &str) -> i32 {
    s.chars().fold(0, |acc, x| if x == '(' { acc + 1 } else if x == ')' { acc - 1 } else { acc })
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
}

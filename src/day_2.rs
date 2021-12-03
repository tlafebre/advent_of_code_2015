use std::str::FromStr;
use std::fs;

fn get_instructions() -> String {
    let filename = "data/day_2.txt";
    
    fs::read_to_string(filename).unwrap()
}

pub fn solve_part1() -> u32 {
    let instructions = get_instructions();

    instructions.lines().fold(0, |acc, x| acc + calculate_wrapping_paper(x))
}

fn get_dimensions(s: &str) -> (u32, u32, u32) {
    let v: Vec<u32> = s.split("x").map(|s| u32::from_str(s).unwrap()).collect();

    (v[0], v[1], v[2])
}

fn area_smallest_side(dimensions: (u32, u32, u32)) -> u32 {
    let (l, w, h) = dimensions;
    let areas = vec![l * w, w * h, l * h];

    areas.iter().min().unwrap().clone()
}

fn calculate_wrapping_paper(s: &str) -> u32 {
    let (l, w, h) = get_dimensions(s);

    (2 * l * w) + (2 * w * h) + (2 * l * h) + area_smallest_side((l, w, h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dimensions() {
        assert_eq!(get_dimensions("2x3x4"), (2, 3, 4));
        assert_eq!(get_dimensions("6x5x4"), (6, 5, 4));
    }

    #[test]
    fn test_area_smallest_side() {
        assert_eq!(area_smallest_side((2, 3, 4)), 6);
        assert_eq!(area_smallest_side((6, 2, 5)), 10);
        assert_eq!(area_smallest_side((1, 1, 10)), 1);
    }

    #[test]
    fn test_calculate_wrapping_paper() {
        assert_eq!(calculate_wrapping_paper("2x3x4"), 58);
        assert_eq!(calculate_wrapping_paper("1x1x10"), 43);
    }

}

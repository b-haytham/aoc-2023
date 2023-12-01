pub fn part1(_input: &str) -> anyhow::Result<String> {
    Ok("Day 2 - Part 1".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_02_part_1() {
        let input = r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#;
        assert_eq!(part1(input).unwrap(), "Day 2 - Part 1".to_string());
    }
}

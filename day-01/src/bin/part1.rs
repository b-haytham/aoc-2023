use day_01::part1::part1;

fn main() -> anyhow::Result<()> {
    let input = include_str!("./input1.txt");
    dbg!(&input);
    let out = part1(input)?;
    dbg!(&out);
    Ok(())
}

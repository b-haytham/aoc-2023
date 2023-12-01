use day_01::part2::part2;

fn main() -> anyhow::Result<()> {
    let input = include_str!("./input2.txt");
    dbg!(&input);

    let out = part2(input)?;
    dbg!(&out);
    Ok(())
}

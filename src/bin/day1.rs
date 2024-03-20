use std::{fs::File, io::{self, BufRead, BufReader}};

fn main() -> io::Result<()> {
    println!("Day 1 example (max, sum): {:?}", most_nutritious_inventories("data/day1example.txt")?);
    println!("Day 1 solution (max, sum): {:?}", most_nutritious_inventories("data/day1input.txt")?);
    Ok(())
}

fn most_nutritious_inventories(filename: &str) -> io::Result<(i32, i32)> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Vec<_>>();

    let mut inventories = lines
        .split(|line| line.as_ref().unwrap().is_empty())
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| line.as_ref().unwrap().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    inventories.sort();
    inventories.reverse();

    Ok((
        inventories[0],
        inventories.iter().take(3).sum::<i32>(),
    ))
}


#[cfg(test)]

#[test]
fn test_example() {
    // TODO refactor not to use IO
    let (max, sum) = most_nutritious_inventories("data/day1example.txt").unwrap();
    assert_eq!(24000, max);
    assert_eq!(45000, sum);
}

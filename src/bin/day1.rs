use std::{fs::File, io::{self, BufRead, BufReader}};

fn main() -> io::Result<()> {
    let lines = BufReader::new(File::open("data/day1input.txt")?).lines();
    let data = lines.map(Result::unwrap).collect::<Vec<_>>();
    let (max, sum) = most_nutritious_inventories(&data).unwrap();
    println!("max: {}, sum: {}", max, sum);
    Ok(())
}

fn most_nutritious_inventories(data: &[String]) -> io::Result<(i32, i32)> {
    let mut inventories = data
        .split(String::is_empty)
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<_>>();

    inventories.sort();
    inventories.reverse();

    Ok((
        inventories[0],
        inventories.iter().take(3).sum(),
    ))
}


#[cfg(test)]

#[test]
fn test_example() {
    let example = [
        "1000",
        "2000",
        "3000",
        "",
        "4000",
        "",
        "5000",
        "6000",
        "",
        "7000",
        "8000",
        "9000",
        "",
        "10000",
    ].map(str::to_string);
    let (max, sum) = most_nutritious_inventories(&example).unwrap();
    assert_eq!(24000, max);
    assert_eq!(45000, sum);
}

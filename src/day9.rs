use aoc_runner_derive::{aoc};

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut sane_blocks: Vec<(usize, usize)> = Vec::new();

    // Parse the input into sane_blocks
    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;
        if count == 0 {
            continue;
        }
        if i % 2 == 0 {
            sane_blocks.push((i / 2 + 1, count));
        } else {
            sane_blocks.push((0, count));
        }
    }

    println!("Initial sane_blocks: {:?}", sane_blocks);

    let mut i = 0;
    while i < sane_blocks.len() {
        if sane_blocks[i].0 != 0 {
            i += 1;
            continue;
        }

        // Process block at index i
        let mut block = sane_blocks[i];
        println!("Processing block {:?} at index {}", block, i);

        let mut j = sane_blocks.len() - 1;
        while j > i {
            let block2 = sane_blocks[j];
            if block2.0 == 0 {
                j -= 1;
                continue;
            }

            println!(
                "Matching block {:?} at index {} with block {:?} at index {}",
                block, i, block2, j
            );

            if block2.1 > block.1 {
                sane_blocks[i].0 = block2.0;
                sane_blocks[j].1 -= block.1;
                println!("1 Moved block {:?}({:?}) into {:?}({:?}) ", j, block2, i, block);
                break;
            } else if block2.1 == block.1 {
                sane_blocks[i].0 = block2.0;
                sane_blocks.remove(j);
                println!("2 Moved block {:?}({:?}) into {:?}({:?}) ", j, block2, i, block);
                break;
            } else {
                sane_blocks[i] = (block2.0, block2.1);
                sane_blocks.insert(i + 1, (0, block.1 - block2.1));
                sane_blocks.remove(j+1);
                println!("3 Moved block {:?}({:?}) into {:?}({:?}) ", j, block2, i, block);
                break;
            }
        }

        i += 1;
    }

    println!("Final sane_blocks: {:?}", sane_blocks);

    let sum = sane_blocks.iter()
        .filter(|x| x.0 != 0)
        .fold(0, |acc, x| acc + (x.0 - 1 ) * x.1 + x.0 * (x.1*x.1 - x.1) / 2);

    sum
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    0
}

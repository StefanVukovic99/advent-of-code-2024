
use aoc_runner_derive::{aoc};

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut sane_blocks: Vec<(usize, usize)> = Vec::new();
    // iterate over enumerated chars
    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;
        if count == 0{
            continue;
        }
        if i%2 == 0 {
            sane_blocks.push((i/2+1, count));
        } else {
            sane_blocks.push((0, count));
        }
    }
    
    println!("{:?}", sane_blocks);

    'outer: for i in 0..sane_blocks.len() {
        let mut block = sane_blocks[i];
        if block.0 != 0 {
            continue;
        }
        println!("Moving into block {:?}({:?})", i, block);

        for j in (i+1..sane_blocks.len()).rev() {
            let mut block2 = sane_blocks[j];
            if block2.0 == 0 {
                continue;
            }
            println!("Moving block {:?}({:?}) into {:?}({:?}) ", j, block2, i, block);
            if block2.1 > block.1 {
                block.0 = block2.0;
                block2.1 -= block.1;
                println!("1 Moved block {:?}({:?}) into {:?}({:?}) ", j, block2, i, block);
                println!("{:?}", sane_blocks);
            } else if block2.1 == block.1 {
                block.0 = block2.0;
                sane_blocks.remove(j);
                println!("2 Moved block {:?}({:?}) into {:?}({:?}) ", j, block2, i, block);
                println!("{:?}", sane_blocks);
            } else {
                sane_blocks.splice(
                    i..i+1, 
                    [
                        (block2.0, block2.1),
                        (block.0, block.1 - block2.1)
                    ]
                );
                sane_blocks.remove(j+1);
                println!("3 Moved block {:?}({:?}) into {:?}({:?}) ", j, block2, i, block);
                println!("{:?}", sane_blocks);
            }

            continue 'outer;
        }
    }

    println!("{:?}", sane_blocks);
    
    0
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    0
}

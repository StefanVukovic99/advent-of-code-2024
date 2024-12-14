use aoc_runner_derive::{aoc};

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut sane_blocks = get_sane_blocks(input);

    let mut i = 0;
    while i < sane_blocks.len() {
        if sane_blocks[i].0 != 0 {
            i += 1;
            continue;
        }

        // Process block at index i
        let mut block = sane_blocks[i];

        let mut j = sane_blocks.len() - 1;
        while j > i {
            let block2 = sane_blocks[j];
            if block2.0 == 0 {
                j -= 1;
                continue;
            }

            if block2.1 > block.1 {
                sane_blocks[i].0 = block2.0;
                sane_blocks[j].1 -= block.1;
                break;
            } else if block2.1 == block.1 {
                sane_blocks[i].0 = block2.0;
                sane_blocks.remove(j);
                break;
            } else {
                sane_blocks[i] = (block2.0, block2.1);
                sane_blocks.insert(i + 1, (0, block.1 - block2.1));
                sane_blocks.remove(j+1);
                break;
            }
        }

        i += 1;
    }


    let mut checskum = 0;
    let mut i = 0;
    for mut block in sane_blocks {
        if block.0 == 0 {
            continue;
        }
        block.0 -= 1;

        let value =  block.1 * block.0 * i + block.0 * block.1*(block.1-1)/2;
        checskum += value;
        i+= block.1;
    }

    checskum
}

fn get_sane_blocks(input: &str) -> Vec<(usize, usize)> {
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

    sane_blocks
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut sane_blocks = get_sane_blocks(input);

    let mut j = sane_blocks.len();
    while j > 0 {
        j -= 1;
        if sane_blocks[j].0 == 0 {
            continue;
        }

        let block2 = sane_blocks[j];

        let mut i = 0;
        while i < j {
            let block = sane_blocks[i];
            if block.0 != 0 {
                i += 1;
                continue;
            }

            if block2.1 > block.1 {
                i+=1;
                continue;
            } else if block2.1 == block.1 {
                sane_blocks[i].0 = block2.0;
                sane_blocks[j].0 = 0;
                break;
            } else {
                sane_blocks[i] = (block2.0, block2.1);
                sane_blocks.insert(i + 1, (0, block.1 - block2.1));
                j+=1;
                sane_blocks[j].0 = 0;
                break;
            }
        }
    }


    let mut checskum = 0;
    let mut i = 0;
    for mut block in sane_blocks {
        if block.0 == 0 {
            i+= block.1;
            continue;
        }
        block.0 -= 1;

        let value =  block.1 * block.0 * i + block.0 * block.1*(block.1-1)/2;
        checskum += value;
        i+= block.1;
    }

    checskum
}
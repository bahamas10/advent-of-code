use std::fs;

use anyhow::Result;

#[derive(Debug)]
struct FilePage {
    pub name: u32,
    pub size: u32,
    pub free_after: u32,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../example-input.txt")?;

    let mut blocks = vec![];
    let mut free = vec![];
    for (i, c) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            blocks.push(c);
            // even
        } else {
            free.push(c);
            // odd
        }
    }

    let mut pages = vec![];
    for (i, block) in blocks.iter().enumerate() {
        let free_after: u32 = *free.get(i).unwrap_or(&'0') as u32 - 48;
        let size: u32 = *block as u32 - 48;
        let name = i as u32;

        let page = FilePage {
            name,
            size,
            free_after,
        };
        pages.push(page)
    }

    for pages.len()..=0 {
    }
    println!("{:#?}", pages);

    Ok(())
}

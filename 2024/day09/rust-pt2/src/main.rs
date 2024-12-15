use std::env;
use std::fs;

use anyhow::{Context, Result};

#[derive(Debug)]
struct FilePage {
    pub name: u64,
    pub size: u64,
    pub free_after: u64,
}

fn print_pages(pages: &[FilePage]) {
    let mut s = String::new();
    for page in pages {
        let name = format!("{}", page.name);
        for _ in 0..page.size {
            s.push_str(&name);
        }
        for _ in 0..page.free_after {
            s.push('.');
        }
    }
    println!("{}", s);
}

fn generate_checksum(pages: &[FilePage]) -> u64 {
    let mut accum = 0;
    let mut index = 0;
    for page in pages {
        for _ in 0..page.size {
            let product = page.name * index;
            accum += product;
            index += 1;
        }
        for _ in 0..page.free_after {
            index += 1;
        }
    }
    accum
}

fn main() -> Result<()> {
    let file = env::args().nth(1).context("need file as arg1")?;
    let input = fs::read_to_string(file)?;

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
        let free_after: u64 = *free.get(i).unwrap_or(&'0') as u64 - 48;
        let size: u64 = *block as u64 - 48;
        let name = i as u64;

        let page = FilePage { name, size, free_after };
        pages.push(page)
    }

    let page_len = pages.len();
    let mut i = page_len - 1;
    'main: while i > 0 {
        print_pages(&pages);
        let move_size = pages[i].size;

        for j in 0..(i - 1) {
            let free_avail = pages[j].free_after;

            if free_avail < move_size {
                continue;
            }

            // we have room!
            // a b (c) d e f (g) h
            // 1. remove g
            // 2. increase f free_after by g.size + g.free_after
            // 3. set g.free_after to c.free_after - g.size
            // 4. set c.free_fater to 0
            // 5. insert g to c+1 location
            let mut g = pages.remove(i);

            let f = &mut pages[i - 1];
            f.free_after += g.size + g.free_after;

            let c = &mut pages[j];
            g.free_after = c.free_after - g.size;

            c.free_after = 0;

            pages.insert(j + 1, g);
            continue 'main;
        }
        i -= 1;
    }
    print_pages(&pages);
    let cksum = generate_checksum(&pages);
    println!("checksum = {}", cksum);

    Ok(())
}

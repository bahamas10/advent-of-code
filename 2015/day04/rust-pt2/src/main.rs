fn main() {
    let mut i = 1;
    let key = "yzbqklnj";
    loop {
        let s = format!("{}{}", key, i);


        let digest = md5::compute(s);
        let hex = format!("{:x}", digest);

        if hex.starts_with("000000") {
            println!("{}", i);
            println!("{:#?}", hex);
            break;
        }

        if i % 1000 == 0 {
            println!("{}", i);
        }

        i += 1;
    }
}

fn main() {
    let mut fibb: i64 = 1;
    let mut prev_fibb: i64 = 0;
    let n = 90;

    for i in 0..n {
        let old_fibb: i64 = if i == 0 {
            0
        } else {
            fibb
        };
        fibb += prev_fibb;
        println!("{}: {}", i, fibb);
        prev_fibb = old_fibb;
    }

}

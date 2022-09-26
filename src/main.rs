fn main() {
    let a = 10;
    let b = 20i32;

    let mut c = 30_i32;

    let d = add(10, 20);

    print!("sum = {}", d)
}

fn add(a: i32, b: i32) -> i32 {
    // 这里不写return 和分号 不然就会当作一个语句了
    a + b
}

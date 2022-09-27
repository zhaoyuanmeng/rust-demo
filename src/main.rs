
// fn main() {
//     let _a = 10;
//     let _b = 20i32;

//     let mut _c = 30_i32;

//     let d = add(10, 20);

//     let (_e, _f): (bool, bool) = (true, false);

//     // ..= 表示序列
//     for i in 1..=5{
//         println!("{}", i);
//     }

//     print!("sum = {}\n", d);
//     print!("e = {}\n", _e);
//     print!("f = {}\n", _f);
// }

// fn add(a: i32, b: i32) -> i32 {
//     // 这里不写return 和分号 不然就会当作一个语句了
//     return a + b;
// }

use num::complex::Complex;

 fn main() {
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a + b;

   println!("{} + {}i", result.re, result.im)
 }


#[link(name = "foo", kind = "static")]
extern "C" {
    // this is rustified prototype of the function from our C library
    fn mytest_func(a: i32, b: i32) -> i32;
}

fn mytest_f(a: i32, b: i32) -> i32 {
    unsafe { mytest_func(a, b) }
}

fn main() {
    let n = mytest_f(12, 23);
    println!("Hello, world! {}", n);
}

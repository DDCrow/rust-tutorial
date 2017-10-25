
fn say_hello_world(name: &str) {
    println!("Say : {}", name);
}
fn sum(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    say_hello_world("HELLO WOLRD");

    println!("5 + 4 = {}", 5 + 4);
    println!("100 + 30 = {}", sum(100, 30));

    let array = [1,2,3,4];
    println!("{}", array[0]);
    println!("{}", array.len());
    println!("{:?}", &array[1..3]);

    let mut vc = vec!["abc","def","ghi","jkl"];
    println!("{}", vc.len());
    for i in vc.iter() {
        println!("{}", i);
    }
    println!("{}", vc.len());
    vc.push("xdd");
    println!("{:?}", vc);
    vc.pop();
    println!("{:?}", vc);
    let mut preallocate_vec = Vec::with_capacity(100);
    for i in 0..100 {
        preallocate_vec.push(i as f64);
    }
    println!("{:?}", preallocate_vec);

    let tuple = ("sianloong", 901207);
    let declare_tuple: (&str, i32) = ("sianloong", 901207);
    println!("{:?}", tuple);
    println!("{:?}", declare_tuple);
    println!("{}", tuple.0);

    let closure_sum = |x: i64, y: i64| x + y;
    println!("25 + 12 = {}", closure_sum(25, 12));
    
    let total = 100;
    let closure_deduct = |x: i64| total - x;
    println!("{} - 88 = {}", total, closure_deduct(88));
}
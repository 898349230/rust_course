fn main() {
    // 基本数据类型，会在栈中拷贝
    let x = 5;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);

    // s1 持有了通过String::from("hello") 创建的值的所有权
    // 当 s1 赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，
    // 这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 会报错
    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // x1 只是引用了存储在二进制中的字符串 "hello, world"，并没有持有所有权。
    let x1: &str = "world";
    let y1 = x1;
    // 不会报错
    println!("x1 = {}, y1 = {},", x1, y1);

    // 克隆（深拷贝），深度复制 String 中堆上的数据， 使用 clone 会极大的降低程序性能
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    println!("函数传值与返回.......");

    let s5 = String::from("happy new day");
    takes_ownership(s5);
    // 这里使用 s5 会报错，以你为 s5 已经被移动到 takes_ownership() 函数中
    // println!("s5: {}", s5);
    let x5 = 5;
    make_copy(x5);
    // 这里不会报错，因为 x5 是被 copy 的
    println!("x5: {}", x5);

    // 
    let s6 = gives_ownership();         // gives_ownership 将返回值移给 s6
    // 
    println!("s6: {}", s6);
    let s7 = String::from("hello");     // s7 进入作用域
    println!("s7: {}", s7);
    let s8 = takes_and_gives_back(s7);  // s7 被移动到takes_and_gives_back 中,它也将返回值移给 s8
    // s7 报错
    // println!("s7: {}", s7);
    println!("s8: {}", s8);

    // 引用与解引用
    println!("引用与解引用。。。。。。。。。。。");

    let x1 = 5;
    // y1 是 x1 的一个引用
    let y1 = &x1;
    assert_eq!(5, x1);
    // 使用 *y1 解引用
    assert_eq!(5,*y1);
    // 这里会报错
    // assert_eq!(5,y1);

    // 不可变引用  
    let s21 = String::from("hello");
    // 将 s21 的引用作为参数传递给 calculate_length() 函数
    let len = calculate_length(&s21);
    println!("s21 {} calculate_length() = {}", s21,len);

    // 可变引用
    let mut s22 = String::from("hello");
    change(&mut s22);
    println!("s22 change() = {}", s22);

    // 可变引用同时只能存在一个。避免数据竞争:
    //     两个或更多的指针同时访问同一数据
    //     至少有一个指针被用来写入数据
    //     没有同步数据访问的机制
    let mut s23 =  String::from("hello");
    let r1 = &mut s23;
    let r2 = &mut s23;
    // 这里会报错，因为 第一个可变借用 r1 必须要持续到 println() 这里，在 r1 创建和最后一次使用之间，我们又尝试创建第二个可变引用 r2。
    // println!("{}, {}", r1, r2);

    // 可变引用与不可变引用不能同时存在
    // let mut s24 = String::from("hello");
    // let r3 = &s24;
    // let r4 = &s24;
    // 这里不可借用
    // let r5 = &mut s24;
    // println!("{}, {}, and {}", r1, r2, r3);

    // 可变引用与不可变引用不能同时存在
    let mut s24 = String::from("hello");
    {
        let r3 = &s24;
        let r4 = &s24;
        println!("{}, {}", r3, r4);
    } // r3, r4 的作用域结束
    // 这里可以借用，因为 r3， r4 的作用域结束在上个 花括号 
    let r5 = &mut s24;
    println!("{}", r5);
}

fn takes_ownership(some_str: String){
    println!("takes_ownership: {}", some_str);
}

fn make_copy(some_int: i32){
    println!("make_copy: {}", some_int);
}

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
    // 调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}

// s 是对 String 的引用（不可变引用）
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变引用
fn change(s:&mut String){
    s.push_str(" world ");
}
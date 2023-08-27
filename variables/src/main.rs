// 引入复数
use num::complex::Complex;
use std::fmt::Debug;

fn main() {
    // 会变错， x 是不可变的
    // let x = 5;
    // mut x 可变
    let mut x:i32 = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    // 未使用的变量要使用 _ 标记，否则编译会提示
    let _z = 5;
    // let y = 10;

    let (a, mut b): (bool, bool) = (true, false);
    println!("a={:?}, b={:?}",  a, b);

    b = true;
    assert_eq!(a, b);

    println!("*****************************************");
    // 赋值语句使用元组，切片，和结构体模式
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    // 声明常量，可以在任意作用域内声明
    const MAX_POINTS: u32 = 100_000;
    println!("const MAX_POINTS = {}", MAX_POINTS);

    // 变量遮蔽
    println!("变量遮蔽。。。。。。。。。");
    // 这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，
    // 而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。
    let x = 5;
    // main 方法作用域内遮蔽
    let x = x + 1;
    {
        // 当前花括号内遮蔽
        let x = x * 2;
        // 12
        println!("The value of x in the inner scope is: {}", x);
    }
    // 6
    println!("The value of x is: {}", x);

    // 数值类型
    println!("数值类型");
    let x1 = (-42.0_f32).sqrt();
    // NaN  not a number
    println!("x1 = {}", x1);

    // 种声明类型方式
    // 自己推断
    let x2 = 1.5;
    let x3:f64 = 1.5;
    // 使用类型后缀
    let x4 = 1.5f64;
    let x5 = 1.590_f64;
    println!("x2 :{}, x3: {}, x4: {}, x5: {}", x2, x3, x4, x5);

    println!("序列。。。。。。。。");
    for i in 1..=5{
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }
    println!("序列结束。。。。。。。。");


    println!("复数计算。。。。。。。。。。");
    let a1 = Complex {re: 2.1, im: -1.2};
    let b1 = Complex::new(11.1,22.2);
    let result = a1 + b1;
    println!("{} + {}i", result.re, result.im);

    println!("函数。。。。。。。。。。。。");
    let add = add(1,2);
    println!("1 + 2 = {}", add);
    dead_end()
}

struct Struct {
    e: i32
}

// 返回一个 i32 类型的值
fn add(i: i32, j:i32) -> i32{
    // 最后一条表达式代表返回值（表达式不用分号结尾），也可以用 return 提前返回
    i + j
}
// ! 作为函数返回类型时，表示该函数永不返回
fn dead_end() -> !{
    panic!("永远不会返回");
}

// 单元类型 ()，是一个零长度的元组。它没啥作用，但是可以用来表达一个函数没有返回值：
// 1.函数没有返回值，那么返回一个 ()
// 2.通过 ; 结尾的表达式返回一个 ()

// 
// report 函数隐式返回一个 ()
fn report<T: Debug>(item: T){
    println!("{:?}", item);
}
// 显式返回 ()
fn clear(text: &mut String) ->(){
    *text = String::from("");
}

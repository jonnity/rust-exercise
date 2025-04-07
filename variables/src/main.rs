const PI_THREE_DIGIT: f64 = 3.14;
// let PI_THREE_DIGIT = 3.14;
// グローバルスコープでは↑とは書けない

fn main() {
    // mutにしておけば、束縛している値を変更可能
    let mut x1 = 5;
    println!("The value of x1 is: {}", x1); // xの値は{}です
    x1 = 6;
    println!("The value of x1 is: {}", x1);

    // mutじゃなくても、shadowingによって、値を変更することは可能
    // ここでは単に{}でブロックを形成しているけど、別関数に渡していると考えると、有用そう
    // ブロック外に副作用がないことを保証できてるから
    let x2 = 5;
    let x2 = x2 + 1;
    {
        let x2 = x2 * 2;
        println!("The value of x2 in the inner scope is: {}", x2);
    }
    println!("The value of x2 is: {}", x2);
}

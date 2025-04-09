fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // ↓はエラーになる
    let sum = x + y;
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt is {}", i.part);

    // 同じ理屈で↓は動かない
    // let mut i: ImportantExcerpt;
    // {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //     i = ImportantExcerpt {
    //         part: first_sentence,
    //     };
    // }
    // println!("Important excerpt is {}", i.part);
}

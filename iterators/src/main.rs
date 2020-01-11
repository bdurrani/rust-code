fn main() {
    println!("Hello, world!");
    let text = "  ponies  \n   giraffes\niguanas  \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    let iter = (0..10).scan(0, |sum, item| {
        *sum += item;
        if *sum > 10 {
            None
        } else {
            Some(item * item)
        }
    });

    assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 4, 9, 16]);

    let input: Vec<String> = vec!["one".to_string(), "two".to_string()]
        .into_iter()
        .collect();
    println!("{:?}", input);
    let test: Vec<&str> = vec!["one", "twp"].into_iter().collect();
    println!("{:?}", test);
    let b = ["one", "two"];
    //        .scan("", |sum, item| { //sum = "";
    //            Some("")
    //        })
    //        .collect();

    //    let message = "To: jimb\r\n\
    //                   From: id\r\n\
    //                   \r\n\
    //                   Oooooh, donuts!!\r\n";
    //
    //    let mut lines = message.lines();
    //
    //    println!("Headers:");
    //    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
    //        println!("header {}", header);
    //    }
    //
    //    println!("\nBody:");
    //    for body in lines {
    //        println!("{}", body);
    //    }
    //
    //    let a = ['1', '2', '3', '∞'];
    //
    //    assert_eq!(a.iter().next(), Some(&'1'));
    //    assert_eq!(a.iter().cloned().next(), Some('1'));

    let a = [
        "Pack ", "my ", "box ", "with ", "five ", "dozen ", "liquor ", "jugs",
    ];

    let pangram = a.iter().fold(String::new(), |mut s, &w| {
        s.push_str(w);
        s
    });
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs");
}

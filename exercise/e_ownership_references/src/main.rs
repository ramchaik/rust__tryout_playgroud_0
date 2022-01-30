fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    println!("1 + 2 = {}, even via references", add(&1, &2));
}

fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("String is plural");
    } else {
        println!("String is singular");
    }
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("a") {
      true
    } else {
      false
    }
}

fn add(a: &i32, b: &i32) -> i32 {
    *a + *b
}

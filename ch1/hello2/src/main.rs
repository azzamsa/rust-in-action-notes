fn greet_world() {
    println!("Hello, world!");

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";

    let regions = [southern_germany, japan];

    // TODO why the book needs `regions.iter()`?
    for region in regions {
        println!("{}", &region)
    }
}

/// The testable version
fn greet_world2() -> [&'static str; 2] {
    println!("Hello, world!");

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";

    [southern_germany, japan]
}

fn main() {
    greet_world();

    let regions = greet_world2();
    for region in regions {
        println!("{}", &region)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet2() {
        let southern_germany = "Grüß Gott!";
        let japan = "ハロー・ワールド";

        let regions = [southern_germany, japan];

        assert_eq!(greet_world2(), regions);
    }
}

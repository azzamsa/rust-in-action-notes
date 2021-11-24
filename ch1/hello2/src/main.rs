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

fn main() {
    greet_world();
}

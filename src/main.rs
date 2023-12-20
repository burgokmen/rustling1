use std::cmp::*;

fn main()  {
    println!("Hello, world!");
  let pair = ('a', 22);
    println!("{} {}", pair.0, pair.1);
    let( a, b) = ("b", 22);

    println!("{} {}", a.to_uppercase() + "d", b + 8);

    println!("{}" ,dice_roll());

    let x = { "out" };
    {
        // this is a different `x`
        let x = "in";

        println!("{}", x);

    }
    println!("{}", x);

    let sss= {
        let x = 11;
        let y= 12;
        y + x;
    };

    let least = min(7,1);
   println!("{}", least);

    struct Vec2 {
        x: f64,
        y: f64,
    }
    let v = Vec2 { x: 3.0, y: 6.0 };
    let Vec2 { y , .. } = v;

println!("{}", y);


}

fn dice_roll() -> i32{
 11
}




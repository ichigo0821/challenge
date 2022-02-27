use std::io;

fn main()
{
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("Triangle");
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

    triangle();
}

fn triangle()
{
    let triangle_angle: u32 = 180;
    let serials = ["First", "Second"];

    let mut total_angle: u32 = 0;

    for serial in serials
    {
        println!("Please enter a {} angle. (degrees)", serial);
        let mut angle = String::new();
        io::stdin().read_line(&mut angle).expect("Failed to read line.");

        let num:u32 = angle.trim().parse().expect("Failed to parse the string.");
        total_angle += num;
        if total_angle > 180
        {
            panic!("total angle is over 180 degrees.");
        }
    }

    println!("The thard angle is {} degrees.", triangle_angle - total_angle);
}


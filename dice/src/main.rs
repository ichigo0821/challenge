use std::io;

fn main()
{
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("The back of a dice");
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

    dice();
}

fn dice()
{
    let dice_sum = 7;
    let mut input = String::new();

    println!("Please input a face of the dice.");
    io::stdin().read_line(&mut input).expect("Faild to read line.");
    let input_num: u32 = input.trim().parse().expect("Faild to parse input.");
    if input_num > 6 || input_num < 1
    {
        panic!("Illegal face of dice.");
    }

    println!("The back of the dice is {}.", dice_sum - input_num);
}


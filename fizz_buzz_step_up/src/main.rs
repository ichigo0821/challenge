fn main()
{
    let limit: u32 = 150;

    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("Fizz Buzz Step up");
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

    for number in 1..(limit + 1)
    {
        fizz_buzz_plus(number);
    }
}

fn fizz_buzz_plus(number: u32)
{
    let term: u32 = 3;

    if number % term == 0 || number.to_string().ends_with(&term.to_string())
    {
        println!("{}!", number);
    }
    else
    {
        println!("{}", number);
    }
}


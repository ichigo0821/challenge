fn main()
{
    let limit: u32 = 100;

    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    println!("Fizz Buzz");
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

    for number in 1..(limit + 1)
    {
        fizz_buzz(number);
    }
}

fn fizz_buzz(number: u32)
{
    // 各種パラメータ
    let multi_fizz: u32 = 3;
    let word_fizz: &str = "Fizz";
    let multi_buzz: u32 = 5;
    let word_buzz: &str = "Buzz";
    let word_fizz_buzz: &str = "FizzBuzz";

    // スコープ範囲を広げるための変数
    let string_number: String;

    let s = match (number % multi_fizz, number % multi_buzz)
    {
        (0, 0) => word_fizz_buzz,
        (0, _) => word_fizz,
        (_, 0) => word_buzz,
        (_, _) => {
            string_number = number.to_string();
            &string_number
        },
    };

    println!("{}", s);
}


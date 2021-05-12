use std::io;

fn main() {
    // 임의의 숫자를 입력 받고(만약 문자열을 입력하면 에러메시지를 내고 다시 입력 받음) 
    // 그 숫자를 20 자리의 xxx,xxx,xxx 형태로 출력하세요.
    // 만약 출력 문자열의 자릿수가 20 자리가 안되면 앞에 '0' 을 붙여주세요.
    // 예를 들어 1234567 을 터미널에서 입력했으면 000000000001,234,567 와 같이 출력하세요.
    println!("임의의 숫자를 입력하세요:");
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("입력 오류");

        let number: u64 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("숫자 형식으로 입력해주세요!");
                continue;
            }
        };

        // 이 연습문제 시나리오에서 출력할 수 있는 가장 큰 숫자는 0999,999,999,999,999이다.
        if number >= 1000000000000000 {
            println!("입력하는 수는 천조보다 작아야 합니다.");
            continue;
        }

        println!("{}", format_number(number));
        break;
    }
}

fn split_thousands (number: u64, formatted: String) -> (u64, String) {
    // number: 3자리마다 분리하려는 수
    // formatted: 수의 일부 또는 전체를 3자리마다 분리해놓은 문자열
    // 최초의 호출에서 formatted는 빈 문자열이며,
    // 그 이후의 재귀 호출에서는 number가 원래의 수에서 차지하는 자리보다 작은 수를 나타낸다.

    // println!("number: {}, formatted: {}", number, formatted);

    let quotient = number / 1000;
    let remainder = number % 1000;
    let result;

    if formatted.len() == 0 {
        // 최초의 호출에서는 1000으로 나눈 나머지 뒤(1~100까지의 자리)에 쉼표를 붙이지 않는다
        result = format!("{}", remainder);
    } else {
        result = format!("{},{}", remainder, formatted);
    }

    if number < 1000 {
        // 분리해야 할 숫자가 3자리 이하면 그대로 반환한다
        (0, result)
    } else {
        // 숫자가 3자리를 초과하면 1천 이하의 3자리 숫자와 그 이상의 값을 따로 반환한다.        
        split_thousands(quotient, result)
    }
}

fn format_number(number: u64) -> String {
    let (_, formatted) = split_thousands(number, String::new());
    let mut result = String::new();

    if formatted.len() < 20 {
        // 출력 문자열의 자릿수가 20자리보다 적으면 0을 패딩한다
        for _ in 0..(20-formatted.len()) {
            result.push('0');
        }
        result += &formatted;
    } else {
        result = formatted;
    }
    result
}
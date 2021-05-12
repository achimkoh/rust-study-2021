use std::io;

fn main() {
    // 터미널에서 문자열을 입력 받아서 그 문자열을 역순으로 출력하세요.
    // 예를 들어 터미널에서 "abbd" 를 입력 받았으면 "dbba"를 출력하세요.
    println!("문자열을 입력하세요:");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("read_line 오류");

    // stdin에서 발생하는 \n 문자를 제거해준다
    // 1안) trim
    // 부수적인 효과: 문자열 끝에 공백이 있으면 그 또한 삭제됨.
    // to_string을 붙여주지 않으면 &str로 type mismatch 발생한다.
    // input = input.trim().to_string();

    // 2안) replace
    user_input = user_input.replace("\n", "");

    let mut output = String::new();
    for character in user_input.chars().rev() {
        output.push(character);
    }
    println!("{}", output);
}

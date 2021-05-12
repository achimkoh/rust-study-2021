fn main() {
	// 정수 1부터 100까지 더하여 화면에 출력하는 프로그램을 작성하세요.
	let mut sum: i32 = 0;
	for number in 1..(100+1) {
		sum += number;
	}
    println!("{}", sum);
}

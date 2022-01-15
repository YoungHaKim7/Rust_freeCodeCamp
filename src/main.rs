use std::thread;
fn main() {
    // 변수 message는 변경이 불가능하므로, 여러 개의 태스크에서 동시에 접근해도 안전하다.
    let message = "Hello";
    let mut threads = Vec::new();
    // `for` 반복문은 `Iterator` trait을 구현하는 어떤 객체에 대해서나 사용할 수 있다.
    for num in 0..10 {
        // `thread::spawn`을 통해 스레드를 생성한다.
        threads.push(thread::spawn(move || {
            // println! 은 매크로이며, 컴파일 시간에 변수의 형 검사가 이루어진다.
            // C나 C++의 단순한 코드 치환 매크로와 달리, 러스트의 매크로는 Scheme과 유사한 구조적 매크로이다.
            println!("{} from task {:?}.", message, num);
        }));
    }
    // 각 스레드가 끝날 때까지 기다린다.
    for thread in threads {
        thread.join().unwrap();
    }
}

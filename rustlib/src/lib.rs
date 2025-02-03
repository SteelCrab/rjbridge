// src/lib.rs

// #[no_mangle] : Rust 컴파일러가 함수 이름을 변경(망글링)하지 않도록 함
// extern "C"  : C 언어와의 ABI(Application Binary Interface)를 맞추기 위함
#[no_mangle]
pub extern "C" fn compute_sum(a: i32, b: i32) -> i32 {
    a + b
}



/*
Linux에서는 librustlib.so
macOS에서는 librustlib.dylib
Windows에서는 rustlib.dll 

생성 파일
 */
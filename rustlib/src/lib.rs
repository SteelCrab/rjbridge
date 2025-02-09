use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ffi::{CStr, CString};//C 문자열을 Rust 문자열로 변환하기 위한 타입
use std::os::raw::c_char; //C 문자열을 위한 타입
use std::sync::{RwLock, Weak, Arc};

//HashMap<String, Weak<CString>>을 사용하여 문자열을 자동으로 재사용
//문자열이 사라지면 Weak 참조하여 자동 삭제(수거) 구현
//Lazy : 전역 변수를 초기화하는 방법
//Arc<CString>을 활용하여 참조 카운팅을 통해 메모리를 안전하게 관리
static R_STRING_POOL : Lazy<RwLock<HashMap<String, Weak<CString>>>>  = 
                        Lazy::new(|| {
                            RwLock::new(HashMap::new())
                        });

// #[no_mangle] : Rust 컴파일러가 함수 이름을 변경(망글링)하지 않도록 함
// extern "C"  : C 언어와의 ABI(Application Binary Interface)를 맞추기 위함
#[no_mangle]
pub extern "C" fn compute_sum(a: i32, b: i32) -> i32 {
    a + b
}

// 문자열을 풀에 저장하고, 동일한 문자열이면 기존의 Arc를 재사용하여 raw pointer를 반환 
#[no_mangle]
pub extern "C" fn pool_str(input: *const c_char) -> *const c_char {
    // c_char의 null 포인터인지 확인
    if input.is_null() {
        return std::ptr::null();
    }
    //입력된 C 문자열을 Rust &str로 변환
    let c_str = unsafe{CStr::from_ptr(input)};
    let rust_str = match c_str.to_str() {
        Ok(rust_str) => rust_str,
        Err(_) => return std::ptr::null(),
    };

    //읽기 lock을 획득하여 R_STRING_POOL에서 문자열을 찾음
    {
        let pool = R_STRING_POOL.read().unwrap();
        if let Some(weak_str) = pool.get(rust_str) {
            if let Some(arc_up) = weak_str.upgrade() {
                return arc_up.as_ptr();
            }
        }
    }
    //존재하지 않으면 새 CString을 생성하고, Arc로 감쌈
        let c_string = match CString::new(rust_str) {
            Ok(cstring) => cstring,
            Err(_) => return std::ptr::null(),
        };

        let arc_string = Arc::new(c_string);
        let ptr = arc_string.as_ptr(); //CString의 raw pointer를 반환

    //쓰기 lock을 획득하여 R_STRING_POOL에 새로운 문자열을 추가(&str,weak<CString>)
    {
        let mut pool = R_STRING_POOL.write().unwrap();
        pool.insert(rust_str.to_owned(), Arc::downgrade(&arc_string));
    }

    //FFI 사용자 참조를 위해 소유권 유지
    std::mem::forget(arc_string);
    ptr
}

//힙 문자열로 생성
#[no_mangle]
pub extern "C" fn new_str( input: *const c_char) -> *const c_char {
    if input.is_null() {
        return std::ptr::null();
        
    }
    //입력된 C 문자열을 Rust String으로 변환
    let c_str = unsafe{CStr::from_ptr(input)};
    let rust_string = match c_str.to_str() {
        Ok(rust_str) => rust_str,
        Err(_) => return std::ptr::null_mut(),
    };

    //CString을 생성하고, raw pointer를 반환
    let c_string = match CString::new(rust_string) {
        Ok(cstring) => cstring,
        Err(_) => return std::ptr::null_mut(),
    };
    //raw pointer :

    c_string.into_raw() //CString의 raw pointer를 반환 
}

// 풀 내부의 항목들 중, 더 이상 강한 참조가 존재하지 않는 항목(약한 참조)들을 정리하여 제거
#[no_mangle]
pub extern "C" fn free_str_pool() {
    let mut pool = R_STRING_POOL.write().unwrap();
    pool.retain(|_, weak| weak.strong_count() > 0);
}
/*
Linux에서는 librustlib.so
macOS에서는 librustlib.dylib
Windows에서는 rustlib.dll 

생성 파일
 */
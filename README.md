# rjbridge
After compiling the Rust code into a dynamic library, Java uses JNA (Java Native Access) to call the library's functions.

# 요구 사항

- **Rust**  
  [Rust 설치 페이지](https://www.rust-lang.org/tools/install)  
  Rust와 Cargo가 설치되어 있어야 합니다.

- **Java JDK**  
  [Java 다운로드 페이지](https://www.oracle.com/java/technologies/javase-downloads.html)  
  Java JDK가 설치되어 있어야 합니다.

- **JNA (Java Native Access)**  
  이 프로젝트는 [JNA](https://github.com/java-native-access/jna)를 사용하여 Rust 동적 라이브러리와 연동합니다.  
  
  ***소스 파일로 설치:***  
     JNA GitHub 저장소(https://github.com/java-native-access/jna)에서 소스를 클론하여 빌드할 수 있습니다.
  
  본 예제에서는 JNA JAR 파일을 `jna-5.16.0/dist/jna.jar` 위치에 두고 사용한다고 가정합니다.

# Rust 동적 라이브러리 빌드

1. `rustlib` 디렉터리로 이동합니다.

   ```bash
   cd rustlib
   cargo build --release
    ```
   빌드가 완료되면, rustlib/target/release/ 디렉터리에 플랫폼에 맞는 동적 라이브러리 파일이 생성됩니다.
    * Linux: librustlib.so
    * macOS: librustlib.dylib
    * Windows: rustlib.dll
# JAVA 소스파일 컴파일
예시 (Linux/Mac):
  ```bash
  cd java
  javac -cp .:/path/to/jna.jar RustLib.java Main.java
  ```
  Windows에서는 클래스패스 구분자를 ;로 사용합니다.
# JAVA 실행
예시 (Linux/Mac):
```bash
# 프로젝트 루트 디렉토리에서 실행 
java -Djna.library.path=./rustlib/target/release/ -cp java:jna-5.16.0/dist/jna.jar Main
```
# 실행 결과 
```bash
Result: 30

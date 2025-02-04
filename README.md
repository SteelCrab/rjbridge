# rjbridge
After compiling the Rust code into a dynamic library, Java uses JNA (Java Native Access) to call the library's functions.

# Requirements

- **Rust**  
  [Rust Installation Page](https://www.rust-lang.org/tools/install)  
  Rust and Cargo must be installed.

- **Java JDK**  
  [Java Download Page](https://www.oracle.com/java/technologies/javase-downloads.html)  
  Java JDK must be installed.

- **JNA (Java Native Access)**  
  This project uses [JNA](https://github.com/java-native-access/jna) to interface with the Rust dynamic library.  
  
  ***Install from Source:***  
     You can clone the source from the JNA GitHub repository (https://github.com/java-native-access/jna) and build it.
  
  This example assumes that the JNA JAR file is located at `jna-5.16.0/dist/jna.jar`.

# Build Rust Dynamic Library

1. Navigate to the `rustlib` directory.

   ```bash
   cd rustlib
   cargo build --release
    ```
   After the build is complete, a platform-specific dynamic library file will be generated in the rustlib/target/release/ directory.
    * Linux: librustlib.so
    * macOS: librustlib.dylib
    * Windows: rustlib.dll
# Compile JAVA Source Files
Example (Linux/Mac):
  ```bash
  cd java
  javac -cp .:/path/to/jna.jar RustLib.java Main.java
  ```
  On Windows, use ; as the classpath separator.
# Run JAVA
Example (Linux/Mac):
```bash
# Run from the project root directory 
java -Djna.library.path=./rustlib/target/release/ -cp java:jna-5.16.0/dist/jna.jar Main
```
# Execution Result 
```bash
Result: 30
Is Same: true
In String: Hello Java - Rust here!
```

// java/RustLib.java
import com.sun.jna.Library;
import com.sun.jna.Native;

public interface RustLib extends Library {
    // 동적 라이브러리 이름은 플랫폼에 따라 적절히 매핑됩니다.
    RustLib INSTANCE = Native.load("rustlib", RustLib.class);

    // Rust에서 구현한 compute_sum 함수와 동일한 시그니처
    int compute_sum(int a, int b);
}

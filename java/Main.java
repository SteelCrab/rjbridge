
import com.sun.jna.Pointer;

public class Main {
    public static void main(String[] args) {
        // Rust의 compute_sum 함수를 호출하여 10과 20의 합을 계산합니다.
        int result = RustLib.INSTANCE.compute_sum(10, 20);
        System.out.println("Result: " + result);

        String str = "Hello Java - Rust here!";
        Pointer ptr1 = RustLib.INSTANCE.pool_str(str);
        Pointer ptr2 = RustLib.INSTANCE.pool_str(str);

        boolean isSame = ptr1.equals(ptr2);
        System.out.println("Is Same: " + isSame);

        String inString = ptr1.getString(0);
        System.out.println("In String: " + inString);

        RustLib.INSTANCE.free_str_pool();
    }
}
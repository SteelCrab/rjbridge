// java/Main.java
public class Main {
    public static void main(String[] args) {
        // Rust의 compute_sum 함수를 호출하여 10과 20의 합을 계산합니다.
        int result = RustLib.INSTANCE.compute_sum(10, 20);
        System.out.println("Result: " + result);
    }
}

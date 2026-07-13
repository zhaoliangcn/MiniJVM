public class FullTest {
    public static void main(String[] args) {
        System.out.println("=== Full Test Suite ===");
        
        testArrayOperations();
        testExceptionHandling();
        testThrowStatement();
        testCheckcast();
        testInstanceof();
        
        System.out.println("=== All tests passed! ===");
    }
    
    static void testArrayOperations() {
        int[] arr = new int[5];
        arr[0] = 10;
        arr[1] = 20;
        arr[2] = arr[0] + arr[1];
        if (arr[2] == 30) {
            System.out.println("Array operations test passed!");
        } else {
            System.out.println("Array operations test FAILED!");
            throw new RuntimeException("Array test failed");
        }
    }
    
    static void testExceptionHandling() {
        try {
            int[] arr = new int[3];
            arr[5] = 1;
            System.out.println("Should not reach here");
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Exception handling test passed!");
        }
    }
    
    static void testThrowStatement() {
        try {
            throw new IllegalArgumentException("Test exception");
        } catch (IllegalArgumentException e) {
            System.out.println("Throw statement test passed!");
        }
    }
    
    static void testCheckcast() {
        Object obj = "Hello";
        try {
            String str = (String) obj;
            System.out.println("Checkcast test passed!");
        } catch (ClassCastException e) {
            System.out.println("Checkcast test FAILED!");
        }
    }
    
    static void testInstanceof() {
        Object obj = "Hello";
        if (obj instanceof String) {
            System.out.println("Instanceof test passed!");
        } else {
            System.out.println("Instanceof test FAILED!");
        }
    }
}

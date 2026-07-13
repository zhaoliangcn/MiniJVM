public class ReturnValueTest {
    public static void main(String[] args) {
        System.out.println("=== Return Value Test ===");
        
        int result = add(10, 20);
        if (result == 30) {
            System.out.println("Integer return test passed!");
        } else {
            System.out.println("Integer return test FAILED! Expected 30, got " + result);
        }
        
        String str = getString();
        if ("Hello World".equals(str)) {
            System.out.println("String return test passed!");
        } else {
            System.out.println("String return test FAILED! Expected 'Hello World', got '" + str + "'");
        }
        
        int max = max(5, 10);
        if (max == 10) {
            System.out.println("Method call with return test passed!");
        } else {
            System.out.println("Method call with return test FAILED! Expected 10, got " + max);
        }
        
        System.out.println("=== All return value tests completed ===");
    }
    
    static int add(int a, int b) {
        return a + b;
    }
    
    static String getString() {
        return "Hello World";
    }
    
    static int max(int a, int b) {
        if (a > b) {
            return a;
        } else {
            return b;
        }
    }
}

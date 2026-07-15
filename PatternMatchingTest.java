public class PatternMatchingTest {
    public static void main(String[] args) {
        Object obj1 = "Hello";
        Object obj2 = 42;
        
        if (obj1 instanceof String s) {
            System.out.println("String length: " + s.length());
        }
        
        if (obj2 instanceof Integer i) {
            System.out.println("Integer value: " + i.intValue());
        }
        
        System.out.println("Pattern matching test passed!");
    }
}

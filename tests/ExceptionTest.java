public class ExceptionTest {
    public static void main(String[] args) {
        System.out.println("Testing exception handling...");
        
        try {
            int[] arr = new int[5];
            arr[10] = 1;
            System.out.println("This should not be printed");
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Caught ArrayIndexOutOfBoundsException");
        }
        
        try {
            String s = null;
            s.length();
        } catch (NullPointerException e) {
            System.out.println("Caught NullPointerException");
        }
        
        System.out.println("Exception handling test passed!");
    }
}
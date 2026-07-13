public class SynchronizationTest {
    private static int counter = 0;
    private static Object lock = new Object();
    
    public static void main(String[] args) {
        System.out.println("=== Synchronization Test ===");
        
        synchronized(lock) {
            counter++;
            if (counter == 1) {
                System.out.println("Synchronized block test 1 passed!");
            }
        }
        
        synchronized(lock) {
            counter++;
            if (counter == 2) {
                System.out.println("Synchronized block test 2 passed!");
            }
        }
        
        Object obj = new Object();
        synchronized(obj) {
            System.out.println("Multiple object lock test passed!");
        }
        
        System.out.println("=== All synchronization tests completed ===");
    }
}
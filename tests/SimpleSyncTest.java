public class SimpleSyncTest {
    public static void main(String[] args) {
        System.out.println("Start");
        Object lock = new Object();
        System.out.println("Created lock");
        synchronized(lock) {
            System.out.println("Inside sync");
        }
        System.out.println("End");
    }
}
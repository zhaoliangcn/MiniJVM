public class StaticFieldTest {
    static int counter = 0;
    
    public static void main(String[] args) {
        counter++;
        if (counter == 1) {
            System.out.println("Static field test passed!");
        } else {
            System.out.println("Static field test FAILED!");
        }
    }
}
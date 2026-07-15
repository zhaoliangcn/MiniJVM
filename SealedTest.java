public class SealedTest {
    public static void main(String[] args) {
        Shape circle = new Circle(3.0);
        Shape rectangle = new Rectangle(4.0, 5.0);
        
        System.out.println("Circle area: " + circle.area());
        System.out.println("Rectangle area: " + rectangle.area());
        
        System.out.println("Sealed classes test passed!");
    }
}

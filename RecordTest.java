public class RecordTest {
    public static void main(String[] args) {
        Point p1 = new Point(3, 4);
        Point p2 = new Point(0, 0);
        System.out.println("p1.x() = " + p1.x());
        System.out.println("p1.y() = " + p1.y());
        System.out.println("Distance = " + p1.distance(p2));
        System.out.println("Record test passed!");
    }
}
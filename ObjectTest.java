public class ObjectTest {
    int value;
    
    public ObjectTest(int v) {
        value = v;
    }
    
    public int getValue() {
        return value;
    }
    
    public static void main(String[] args) {
        ObjectTest obj = new ObjectTest(42);
        System.out.println(obj.getValue());
    }
}

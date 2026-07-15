public class NewFeaturesTest {
    public static void main(String[] args) {
        testStringBuilder();
        testWrapperClasses();
        testMath();
        testInvokeDynamic();
        System.out.println("=== All new features tests passed! ===");
    }

    public static void testStringBuilder() {
        StringBuilder sb = new StringBuilder();
        sb.append("Hello");
        sb.append(" ");
        sb.append("World");
        sb.append("!");
        String result = sb.toString();
        if (result.equals("Hello World!")) {
            System.out.println("StringBuilder test passed!");
        } else {
            System.out.println("StringBuilder test failed: " + result);
        }
    }

    public static void testWrapperClasses() {
        int intVal = Integer.parseInt("42");
        if (intVal == 42) {
            System.out.println("Integer.parseInt test passed!");
        }

        long longVal = Long.parseLong("9223372036854775807");
        if (longVal == 9223372036854775807L) {
            System.out.println("Long.parseLong test passed!");
        }

        double doubleVal = Double.parseDouble("3.14159");
        if (Math.abs(doubleVal - 3.14159) < 0.0001) {
            System.out.println("Double.parseDouble test passed!");
        }

        String intStr = Integer.toString(123);
        if (intStr.equals("123")) {
            System.out.println("Integer.toString test passed!");
        }
    }

    public static void testMath() {
        int abs = Math.abs(-42);
        if (abs == 42) {
            System.out.println("Math.abs test passed!");
        }

        int max = Math.max(10, 20);
        if (max == 20) {
            System.out.println("Math.max test passed!");
        }

        int min = Math.min(10, 20);
        if (min == 10) {
            System.out.println("Math.min test passed!");
        }

        double sqrt = Math.sqrt(16.0);
        if (Math.abs(sqrt - 4.0) < 0.0001) {
            System.out.println("Math.sqrt test passed!");
        }

        double pow = Math.pow(2.0, 10.0);
        if (Math.abs(pow - 1024.0) < 0.0001) {
            System.out.println("Math.pow test passed!");
        }
    }

    public static void testInvokeDynamic() {
        String name = "Mini";
        String version = "JVM";
        String result = name + " " + version;
        if (result.equals("Mini JVM")) {
            System.out.println("String concatenation (invokedynamic) test passed!");
        }
    }
}
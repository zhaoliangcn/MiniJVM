public sealed interface Shape permits Circle, Rectangle {
    double area();
}

public record Point(int x, int y) {
    public int distance(Point other) {
        int dx = x - other.x;
        int dy = y - other.y;
        return (int) Math.sqrt(dx * dx + dy * dy);
    }
}
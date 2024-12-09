public record Vector(int row, int col) {
    public Vector(Point a, Point b) {
        this(b.row() - a.row(), b.col() - a.col());
    }
}

public record Point(int row, int col) {
    public Point add(Vector vector) {
        return new Point(row + vector.row(), col + vector.col());
    }

    public Point subtract(Vector vector) {
        return new Point(row - vector.row(), col - vector.col());
    }
}

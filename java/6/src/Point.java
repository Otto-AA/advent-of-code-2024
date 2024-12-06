public record Point(int row, int col) {

    public Point add(Vector vector) {
        return new Point(row + vector.row(), col + vector.col());
    }

    // true if it is next to this point (vertical, horizontal or diagonal)
    public boolean isAdjacent(Point point) {
        return this != point && Math.abs(row - point.row()) <= 1 && Math.abs(col - point.col()) <= 1;
    }
}

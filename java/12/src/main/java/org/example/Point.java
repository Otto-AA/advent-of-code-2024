package org.example;

public record Point(int row, int col) {
    public Point add(Vector vector) {
        return new Point(row + vector.row(), col + vector.col());
    }

    public Point subtract(Vector vector) {
        return new Point(row - vector.row(), col - vector.col());
    }

    public Point[] directNeighbours() {
        return new Point[]{
                new Point(row + 1, col),
                new Point(row - 1, col),
                new Point(row, col + 1),
                new Point(row, col - 1),
        };
    }

    // inclusive check
    // if start = (0, 0) and end = (5, 5)
    // then (4, 5) is in range
    // and (4, 6) is not in range
    public boolean isInSquareRange(Point start, Point end) {
        return row >= start.row && row <= end.row && col >= start.col && col <= end.col;
    }
}


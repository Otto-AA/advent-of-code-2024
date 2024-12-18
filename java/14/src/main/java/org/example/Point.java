package org.example;

public record Point(int x, int y) {
    public Point add(Vector vector) {
        return new Point(x + vector.x(), y + vector.y());
    }
}

package org.example;

public record Rectangle(Point topLeft, Point bottomRight) {
    public int height() {
        return bottomRight.y() - topLeft.y() + 1;
    }

    public int width() {
        return bottomRight.x() - topLeft.x() + 1;
    }
}

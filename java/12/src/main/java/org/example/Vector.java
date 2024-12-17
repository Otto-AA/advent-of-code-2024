package org.example;

public record Vector(int row, int col) {
    // Create a vector along a side
    // TOP is the opposite direction of BOTTOM (and LEFT for RIGHT)
    public static Vector from(Side side) {
        switch (side) {
            case TOP -> {
                return new Vector(0, 1);
            }
            case RIGHT -> {
                return new Vector(1, 0);
            }
            case BOTTOM -> {
                return new Vector(0, -1);
            }
            case LEFT -> {
                return new Vector(-1, 0);
            }
        }
        throw new RuntimeException("Invalid side: " + side);
    }

    public Vector inverse() {
        return new Vector(-row, -col);
    }
}

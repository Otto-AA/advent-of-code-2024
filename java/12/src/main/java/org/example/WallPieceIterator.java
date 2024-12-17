package org.example;

import java.util.Iterator;

public class WallPieceIterator implements Iterator<WallPiece> {
    private Point nextPoint;
    private final Point lastPoint;
    private final Side side;
    // normalized vector (either 1, 0 or -1)
    private final Vector direction;

    public WallPieceIterator(Wall wall) {
        nextPoint = wall.start();
        lastPoint = wall.end();
        side = wall.side();

        int directionRow = wall.end().row() - wall.start().row();
        if (directionRow != 0) {
            directionRow /= Math.abs(directionRow);
        }
        int directionCol = wall.end().col() - wall.start().col();
        if (directionCol != 0) {
            directionCol /= Math.abs(directionCol);
        }
        direction = new Vector(directionRow, directionCol);
    }

    @Override
    public boolean hasNext() {
        return nextPoint != null;
    }

    @Override
    public WallPiece next() {
        var point = nextPoint;
        var wallPiece = new WallPiece(point, side);

        if (lastPoint.equals(nextPoint)) {
            nextPoint = null;
        } else {
            nextPoint = point.add(direction);
        }
        return wallPiece;
    }
}

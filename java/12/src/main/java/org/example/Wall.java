package org.example;

import java.util.Iterator;

public record Wall(Point start, Point end, Side side) implements Iterable<WallPiece> {
    @Override
    public Iterator<WallPiece> iterator() {
        return new WallPieceIterator(this);
    }
}

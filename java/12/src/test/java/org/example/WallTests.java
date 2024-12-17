package org.example;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

public class WallTests {
    @Test
    void iterationYieldsPieces() {
        var wall = new Wall(new Point(1, 5), new Point(3, 5), Side.RIGHT);

        assertIterableEquals(List.of(
                new WallPiece(new Point(1, 5), Side.RIGHT),
                new WallPiece(new Point(2, 5), Side.RIGHT),
                new WallPiece(new Point(3, 5), Side.RIGHT)), wall);
    }

    @Test
    void canIterateDownwards() {
        var wall = new Wall(new Point(5, 5), new Point(3, 5), Side.RIGHT);

        assertIterableEquals(List.of(
                new WallPiece(new Point(5, 5), Side.RIGHT),
                new WallPiece(new Point(4, 5), Side.RIGHT),
                new WallPiece(new Point(3, 5), Side.RIGHT)), wall);
    }
}

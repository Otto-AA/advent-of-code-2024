package org.example;

import static org.junit.jupiter.api.Assertions.*;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.Set;

public class RegionSidesTests {
    @Test
    void emptyRegion() {
        var region = new Region<>(1234, Set.of());

        assertEquals(0, region.countSides());
    }

    @Test
    void singleFieldRegion() {
        var region = new Region<>(1234, Set.of(new Point(1, 2)));

        assertEquals(4, region.countSides());
    }

    @Test
    void lineRegion() {
        var region = new Region<>(1234, Set.of(new Point(0, 0), new Point(0, 1), new Point(0, 2)));

        assertEquals(4, region.countSides());
    }

    @Test
    void squareRegion() {
        var region = new Region<>(1234, Set.of(new Point(0, 0), new Point(0, 1), new Point(1, 0), new Point(1, 1)));

        assertEquals(4, region.countSides());
    }

    @Test
    void triangleRegion() {
        // X X
        // X
        var region = new Region<>(1234, Set.of(new Point(0, 0), new Point(0, 1), new Point(1, 0)));

        assertEquals(6, region.countSides());
    }

    @Test
    void squareWithHoleRegion() {
        // X X X
        // X   X
        // X X X
        var points = HashSet.<Point>newHashSet(8);
        for (int row = 0; row < 3; row++) {
            for (int col = 0; col < 3; col++) {
                if (row != 1 || col != 1) {
                    points.add(new Point(row, col));
                }
            }
        }
        var region = new Region<>(1234, points);

        assertEquals(8, region.countSides());
    }
}

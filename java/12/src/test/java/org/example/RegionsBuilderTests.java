package org.example;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Set;

public class RegionsBuilderTests {
    @Test
    void buildsSingleRegion() {
        var data = List.of(List.of(1, 1), List.of(1, 1));

        var regions = RegionsBuilder.from(data).build();

        assertEquals(1, regions.size());
    }

    @Test
    void buildsSingleRegionWithFourPoints() {
        var data = List.of(List.of(1, 1), List.of(1, 1));

        var regions = RegionsBuilder.from(data).build();
        var points = regions.getFirst().points();

        var expectedPoints = Set.of(new Point(0, 0), new Point(0, 1), new Point(1, 0), new Point(1, 1));
        assertEquals(expectedPoints, points);
    }

    @Test
    void buildsMultipleRegions() {
        var data = List.of(List.of(1, 1, 2), List.of(1, 2, 2));

        var regions = RegionsBuilder.from(data).build();

        assertEquals(2, regions.size());
    }
}

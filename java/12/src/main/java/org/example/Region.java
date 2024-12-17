package org.example;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Set;

public record Region<T>(T value, Set<Point> points) {

    public long calculatePerimeter() {
        var perimeter = 0L;
        for (var p : points) {
            var actualNeighbours = Arrays.stream(p.directNeighbours()).filter(points::contains).count();
            // a point has a maximum of 4 neighbours
            // every side without a neighbour is on the perimeter
            perimeter += (4 - actualNeighbours);
        }
        return perimeter;
    }

    public int countSides() {
        var iterator = new RegionSidesIterator<>(this);
        var sides = 0;
        while (iterator.hasNext()) {
            iterator.next();
            sides++;
        }
        return sides;
    }

    public long area() {
        return points.size();
    }
}

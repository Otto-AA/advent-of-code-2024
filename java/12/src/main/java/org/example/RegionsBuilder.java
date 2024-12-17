package org.example;

import java.util.*;
import java.util.stream.Collectors;

public class RegionsBuilder<T> {
    private final List<List<T>> values;
    private final Point minPoint = new Point(0, 0);
    private final Point maxPoint;

    private RegionsBuilder(List<List<T>> values) {
        this.values = values;
        maxPoint = new Point(values.size() - 1, values.getFirst().size() - 1);
    }

    public static <T> RegionsBuilder<T> from(List<List<T>> values) {
        return new RegionsBuilder<>(values);
    }

    public ArrayList<Region<T>> build() {
        var regions = new ArrayList<Region<T>>();
        int rows = values.size();
        int cols = values.getFirst().size();

        var coveredPoints = HashSet.<Point>newHashSet(rows * cols);

        for (int row = 0; row < rows; row++) {
            for (int col = 0; col < cols; col++) {
                var point = new Point(row, col);
                if (coveredPoints.contains(point)) {
                    continue;
                }

                var area = getSameValueArea(point);
                coveredPoints.addAll(area);
                regions.add(new Region<>(getValue(point), area));
            }
        }

        return regions;
    }

    private Set<Point> getSameValueArea(Point point) {
        var area = new HashSet<Point>();
        area.add(point);
        var neighbours = getSameValueNeighbours(point);

        while (!neighbours.isEmpty()) {
            var neighbour = neighbours.iterator().next();
            neighbours.remove(neighbour);

            // only add neighbours once per point
            if (area.add(neighbour)) {
                neighbours.addAll(getSameValueNeighbours(neighbour));
            }
        }

        return area;
    }

    private Set<Point> getSameValueNeighbours(Point point) {
        var value = getValue(point);
        return Arrays.stream(point.directNeighbours())
                .filter(this::isInRange)
                .filter(p -> value.equals(getValue(p)))
                .collect(Collectors.toSet());
    }

    private boolean isInRange(Point point) {
        return point.isInSquareRange(minPoint, maxPoint);
    }

    private T getValue(Point point) {
        return values.get(point.row()).get(point.col());
    }
}

package org.example;

import java.util.*;
import java.util.stream.Collectors;

public class RegionSidesIterator<T> implements Iterator<Wall> {
    private final Set<Point> points;
    private final Set<WallPiece> wallPieces;

    public RegionSidesIterator(Region<T> region) {
        this.points = region.points();
        this.wallPieces = points.stream().flatMap(p -> freeSides(p).stream().map(s -> new WallPiece(p, s))).collect(Collectors.toSet());
    }

    @Override
    public boolean hasNext() {
        return !wallPieces.isEmpty();
    }

    @Override
    public Wall next() {
        var wallPiece = wallPieces.iterator().next();
        var wall = getContinuousWall(wallPiece);
        removeWallFromPieces(wall);

        return wall;
    }

    private Wall getContinuousWall(WallPiece wallPiece) {
        var start = wallStart(wallPiece);
        var end = wallEnd(wallPiece);
        return new Wall(start, end, wallPiece.side());
    }

    private void removeWallFromPieces(Wall wall) {
        for (var wallPiece : wall) {
            wallPieces.remove(wallPiece);
        }
    }

    private Point wallStart(WallPiece wallPiece) {
        var direction = Vector.from(wallPiece.side()).inverse();
        return findLastPointInDirection(wallPiece.point(), wallPiece.side(), direction);
    }

    private Point wallEnd(WallPiece wallPiece) {
        var direction = Vector.from(wallPiece.side());
        return findLastPointInDirection(wallPiece.point(), wallPiece.side(), direction);
    }

    private Point findLastPointInDirection(Point start, Side freeSide, Vector direction) {
        var lastPoint = start;

        // go one step too far
        while (points.contains(lastPoint) && freeSides(lastPoint).contains(freeSide)) {
            lastPoint = lastPoint.add(direction);
        }

        return lastPoint.subtract(direction);
    }

    private Set<Side> freeSides(Point point) {
        var sides = HashSet.<Side>newHashSet(4);
        if (!points.contains(new Point(point.row() + 1, point.col()))) {
            sides.add(Side.BOTTOM);
        }
        if (!points.contains(new Point(point.row() - 1, point.col()))) {
            sides.add(Side.TOP);
        }
        if (!points.contains(new Point(point.row(), point.col() + 1))) {
            sides.add(Side.RIGHT);
        }
        if (!points.contains(new Point(point.row(), point.col() - 1))) {
            sides.add(Side.LEFT);
        }

        return sides;
    }
}

package org.example;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var scanner = new Scanner(System.in);
        scanner.useDelimiter("\\n");

        var width = Integer.parseInt(args[0]);
        var height = Integer.parseInt(args[1]);
        var simulation = parse(scanner, width, height);
        System.out.println(partOne(simulation));
        simulation.rewind(100);
        partTwo(simulation);
    }

    public static Simulation parse(Iterator<String> lines, int width, int height) {
        return new Simulation(height, width, parseRobots(lines));
    }

    public static List<Robot> parseRobots(Iterator<String> lines) {
        var robots = new ArrayList<Robot>();

        while (lines.hasNext()) {
            var line = lines.next();
            var parts = line.split(" ");
            var positionParts = parts[0].split(",");
            var velocityParts = parts[1].split(",");
            var position = new Point(Integer.parseInt(positionParts[0].split("=")[1]), Integer.parseInt(positionParts[1]));
            var velocity = new Vector(Integer.parseInt(velocityParts[0].split("=")[1]), Integer.parseInt(velocityParts[1]));
            robots.add(new Robot(position, velocity));
        }

        return robots;
    }

    public static long partOne(Simulation simulation) {
        simulation.fastForward(100);
        var quadrants = getQuadrants(simulation.currentMap());

        var safetyFactor = 1L;

        for (var quadrant : quadrants) {
            var robots = quadrant.stream().flatMapToInt(row -> row.stream().mapToInt(v -> v)).sum();
            safetyFactor *= robots;
        }

        return safetyFactor;
    }

    public static <T> List<List<List<T>>> getQuadrants(List<List<T>> map) {
        var height = map.size();
        var width = map.getFirst().size();
        var quadrantHeight = height / 2;
        var quadrantWidth = width / 2;

        var rectangles = List.of(
                new Rectangle(new Point(0, 0), new Point(quadrantWidth - 1, quadrantHeight - 1)),
                new Rectangle(new Point(width - quadrantWidth, 0), new Point(width - 1, quadrantHeight - 1)),
                new Rectangle(new Point(0, height - quadrantHeight), new Point(quadrantWidth - 1, height - 1)),
                new Rectangle(new Point(width - quadrantWidth, height - quadrantHeight), new Point(width - 1, height - 1))
        );

        return rectangles.stream().map(r -> getRectangle(map, r)).toList();
    }

    public static <T> List<List<T>> getRectangle(List<List<T>> map, Rectangle rectangle) {
        var result = new ArrayList<List<T>>();
        for (int row = rectangle.topLeft().y(); row <= rectangle.bottomRight().y(); row++) {
            var newRow = new ArrayList<T>();
            for (int col = rectangle.topLeft().x(); col <= rectangle.bottomRight().x(); col++) {
                newRow.add(map.get(row).get(col));
            }
            result.add(newRow);
        }
        return result;
    }

    public static void partTwo(Simulation simulation) {
        // a tree likely has a trunk
        var expectedShape = new Rectangle(new Point(0, 0), new Point(1, 10));

        for (int i = 1; i <= 10000; i++) {
            simulation.fastForward(1);
            var map = simulation.currentMap();

            if (findFilledRectangle(map, expectedShape)) {
                System.out.println(i);
                map.forEach(row -> System.out.println(row.stream().map(n -> n == 0 ? "." : "" + n).toList()));
            }
        }
    }

    public static boolean findFilledRectangle(List<List<Integer>> map, Rectangle rectangle) {
        int height = map.size();
        int width = map.getFirst().size();

        for (int y = 0; y < height - rectangle.height(); y++) {
            for (int x = 0; x < width - rectangle.width(); x++) {
                var vector = new Vector(x, y);
                var movedRectangle = new Rectangle(rectangle.topLeft().add(vector), rectangle.bottomRight().add(vector));
                var area = getRectangle(map, movedRectangle);
                if (isFilled(area)) {
                    return true;
                }
            }
        }
        return false;
    }

    private static boolean isFilled(List<List<Integer>> map) {
        return !mapContains(map, 0);
    }

    private static <T> boolean mapContains(List<List<T>> map, T value) {
        return map.stream().anyMatch(row -> row.contains(value));
    }
}
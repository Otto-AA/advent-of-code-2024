import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var lines = readInput();
        var area = new Area(lines.getFirst().length(), lines.size());
        Point guardStart = null;
        Vector guardDirection = null;

        for (int row = 0; row < area.height(); row++) {
            for (int col = 0; col < area.width(); col++) {
                var point = new Point(row, col);
                switch (lines.get(row).charAt(col)) {
                    case '#':
                        area.addObstacle(point);
                        break;
                    case '<':
                        guardStart = point;
                        guardDirection = new Vector(0, -1);
                        break;
                    case '^':
                        guardStart = point;
                        guardDirection = new Vector(-1, 0);
                        break;
                    case '>':
                        guardStart = point;
                        guardDirection = new Vector(0, 1);
                        break;
                    case 'v':
                        guardStart = point;
                        guardDirection = new Vector(1, 0);
                        break;
                }
            }
        }

        if (guardStart == null) {
            throw new RuntimeException("Could not locate guard");
        }
        runPart1(area, new Guard(area, guardDirection, guardStart));
        runPart2(area, new Guard(area, guardDirection, guardStart));
    }

    private static List<String> readInput() {
        var lines = new ArrayList<String>();
        var scanner = new Scanner(System.in);
        while (scanner.hasNextLine()) {
            lines.add(scanner.nextLine());
        }
        return lines;
    }

    private static void runPart1(Area area, Guard guard) {
        var passedPoints = new HashSet<Point>();

        while (area.inRange(guard.location)) {
            passedPoints.add(guard.location);
            guard.step();
        }

        System.out.printf("Guard passed points: %d\n", passedPoints.size());
    }

    private static void runPart2(Area area, Guard guard) {
        int possibilitiesCount = 0;
        for (int row = 0; row < area.height(); row++) {
            for (int col = 0; col < area.width(); col++) {
                var point = new Point(row, col);
                if (area.isObstacle(point)) {
                    continue;
                }
                area.addObstacle(point);
                if (isStuckInLoop(area, new Guard(area, guard.direction, guard.location))) {
                    possibilitiesCount++;
                }
                area.removeObstacle(point);
            }
        }

        System.out.printf("Possible obstacles: %d\n", possibilitiesCount);

    }

    private static boolean isStuckInLoop(Area area, Guard guard) {
        var passedPoints = new HashSet<PointWithVector>();

        while (area.inRange(guard.location)) {
            passedPoints.add(new PointWithVector(guard.location, guard.direction));
            guard.step();
            if (passedPoints.contains(new PointWithVector(guard.location, guard.direction))) {
                return true;
            }
        }
        return false;
    }
}
import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;
import java.util.stream.Stream;

public class TopographicMap {
    private final List<List<Integer>> map;
    private final int rows;
    private final int cols;

    public TopographicMap(List<List<Integer>> map, int height, int width) {
        this.map = map;
        this.rows = height;
        this.cols = width;
    }

    public int mapHeight() {
        return rows;
    }

    public int mapWidth() {
        return cols;
    }

    public int heightAt(Point point) {
        return map.get(point.row()).get(point.col());
    }

    public boolean inRange(Point point) {
        return point.row() >= 0 && point.row() < rows && point.col() >= 0 && point.col() < cols;
    }

    public List<Point> adjacentPoints(Point point) {
        return Stream.of(
                point.add(new Vector(0, 1)),
                point.add(new Vector(1, 0)),
                point.add(new Vector(0, -1)),
                point.add(new Vector(-1, 0))
        ).filter(this::inRange).toList();
    }

    public List<Point> findByHeight(int height) {
        var points = new ArrayList<Point>();
        for (int row = 0; row < rows; row++) {
            for (int col = 0; col < cols; col++) {
                var point = new Point(row, col);
                if (heightAt(point) == height) {
                    points.add(point);
                }
            }
        }
        return points;
    }
}

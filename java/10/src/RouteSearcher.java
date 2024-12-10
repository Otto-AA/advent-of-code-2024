import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class RouteSearcher {
    private final TopographicMap map;
    // maps each point to a list of start points that it can reach
    private final List<List<List<Point>>> reachablePoints;
    private final int bottom;
    private final int top;

    public RouteSearcher(TopographicMap map, int bottom, int top) {
        this.map = map;
        this.bottom = bottom;
        this.top = top;
        reachablePoints = new ArrayList<>();
        for (int row = 0; row < map.mapHeight(); row++) {
            var list = new ArrayList<List<Point>>();
            reachablePoints.add(list);
            for (int col = 0; col < map.mapWidth(); col++) {
                list.add(new ArrayList<>());
            }
        }
    }

    public void computeReachability() {
        // mark start points as reachable
        map.findByHeight(bottom).forEach(p -> markReachable(p, p));

        // mark all start points that are reachable at height i
        // also as reachable from neighbours at height (i + 1)
        for (int i = bottom; i < top; i++) {
            for (var point : map.findByHeight(i)) {
                var reachableStartPoints = reachableStartPoints(point);
                for (var neighbour : map.adjacentPoints(point)) {
                    if (map.heightAt(neighbour) != i + 1) {
                        continue;
                    }

                    reachableStartPoints.forEach(p -> markReachable(neighbour, p));
                }
            }
        }
    }

    public List<Point> getReachableStartPoints(Point from) {
        return reachableStartPoints(from);
    }

    public Set<Point> getUniqueReachableStartPoints(Point from) {
        return new HashSet<>(reachableStartPoints(from));
    }

    private void markReachable(Point from, Point startPoint) {
        reachablePoints.get(from.row()).get(from.col()).add(startPoint);
    }

    private List<Point> reachableStartPoints(Point from) {
        return reachablePoints.get(from.row()).get(from.col());
    }

    public void printReachabilityScores() {
        for (int row = 0; row < map.mapHeight(); row++) {
            for (int col = 0; col < map.mapWidth(); col++) {
                System.out.printf("[%d]", getReachableStartPoints(new Point(row, col)).size());
            }
            System.out.println();
        }
    }


}

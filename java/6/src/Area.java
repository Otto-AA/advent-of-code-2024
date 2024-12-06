public class Area {
    private final boolean[][] area;

    public Area(int width, int height) {
        this.area = new boolean[height][];
        for (int row = 0; row < height; row++) {
            this.area[row] = new boolean[width];
        }
    }

    public void addObstacle(Point point) {
        this.area[point.row()][point.col()] = true;
    }

    public void removeObstacle(Point point) {
        this.area[point.row()][point.col()] = false;
    }
    
    public boolean isObstacle(Point point) {
        return area[point.row()][point.col()];
    }

    // first obstacle in target direction
    // includes starting point for obstacle check
    // returns null if no obstacle exists in this direction
    public Point findNextObstacle(Point start, Vector direction) {
        var current = start;
        while (inRange(current)) {
            if (isObstacle(current)) {
                return current;
            }
            current = current.add(direction);
        }
        return null;
    }

    public boolean inRange(Point point) {
        return point.row() >= 0 && point.row() < height() && point.col() >= 0 && point.col() < width();

    }

    public int width() {
        if (height() == 0) {
            return 0;
        }
        return area[0].length;
    }

    public int height() {
        return area.length;
    }
}

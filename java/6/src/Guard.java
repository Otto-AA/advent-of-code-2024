public class Guard {
    private final Area area;
    public Vector direction;
    public Point location;

    public Guard(Area area, Vector direction, Point location) {
        this.area = area;
        this.direction = direction;
        this.location = location;
    }

    // updates and returns new location
    public void step() {
        turnBeforeObstacles();
        stepForward();
    }

    // turn right until all obstacles are dodged
    private void turnBeforeObstacles() {
        var nextObstacle = area.findNextObstacle(location, direction);
        while (nextObstacle != null && location.isAdjacent(nextObstacle)) {
            direction = direction.turnRight();
            nextObstacle = area.findNextObstacle(location, direction);
        }
    }

    private void stepForward() {
        location = location.add(direction);
    }
}

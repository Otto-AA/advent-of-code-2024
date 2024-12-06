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
        location = computeMove();
    }

    // turn right until all obstacles are dodged
    private void turnBeforeObstacles() {
        var nextPoint = computeMove();
        while (area.inRange(nextPoint) && area.isObstacle(nextPoint)) {
            direction = direction.turnRight();
            nextPoint = computeMove();
        }
    }

    private Point computeMove() {
        return location.add(direction);
    }
}

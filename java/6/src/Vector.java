public record Vector(int row, int col) {
    public Vector turnRight() {
        return new Vector(col, -row);
    }
}

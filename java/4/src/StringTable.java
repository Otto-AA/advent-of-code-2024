import java.util.LinkedList;
import java.util.List;

public class StringTable {
    private final List<String> data;
    private final int rows;
    private final int cols;

    public StringTable(List<String> table) {
        data = table;
        rows = table.size();
        cols = table.getFirst().length();
    }

    public char at(int row, int col) {
        return data.get(row).charAt(col);
    }

    public boolean has(int row, int col) {
        return row >= 0 && row < rows && col >= 0 && col < cols;
    }

    public Iterable<Cell> cellsIterable() {
        List<Cell> cells = new LinkedList<>();

        for (int row = 0; row < rows; row++) {
            for (int col = 0; col < cols; col++) {
                cells.add(new Cell(row, col));
            }
        }
        return cells;
    }
}

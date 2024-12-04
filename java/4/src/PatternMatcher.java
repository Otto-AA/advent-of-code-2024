import java.util.Iterator;
import java.util.LinkedList;

public class PatternMatcher {
    private final StringTable table;

    public PatternMatcher(StringTable table) {
        this.table = table;
    }

    public Iterable<Cell> findMatches(StringPattern pattern) {
        var matches = new LinkedList<Cell>();
        for (var cell : table.cellsIterable()) {
            if (matches(pattern, cell)) {
                matches.add(cell);
            }
        }

        return matches;
    }

    public boolean matches(StringPattern pattern, Cell start) {
        for (var patternChar : pattern) {
            var row = start.row() + patternChar.rowOffset();
            var col = start.col() + patternChar.colOffset();

            if (!table.has(row, col)) {
                return false;
            }
            if (patternChar.character() != table.at(row, col)) {
                return false;
            }
        }

        return true;
    }
}

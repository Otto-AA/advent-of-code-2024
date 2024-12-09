import java.util.*;

public class Antennas {
    private final Map<Character, List<Point>> positions = new HashMap<>();

    public void add(char type, int row, int col) {
        var antennasOfType = positions.computeIfAbsent(type, (k) -> new ArrayList<>());
        antennasOfType.add(new Point(row, col));
    }

    public Iterable<Character> iterateTypes() {
        return positions.keySet();
    }

    public Iterable<ListCombinationsIterator.Combination<Point>> iterateLocationTuples(char type) {
        var antennasOfType = positions.get(type);
        if (antennasOfType == null) {
            return Collections::emptyIterator;
        }

        return () -> new ListCombinationsIterator<>(antennasOfType);
    }
}

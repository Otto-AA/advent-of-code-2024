import java.util.Iterator;
import java.util.List;

public class ListCombinationsIterator<T> implements Iterator<ListCombinationsIterator.Combination<T>> {
    private final List<T> list;
    private final int size;
    private int i = 0;
    private int j = 1;

    public ListCombinationsIterator(List<T> list) {
        this.list = list;
        size = list.size();
    }

    @Override
    public boolean hasNext() {
        return i < size && j < size;
    }

    @Override
    public Combination<T> next() {
        var value = new Combination<>(list.get(i), list.get(j));
        updateIndices();
        return value;
    }

    private void updateIndices() {
        j += 1;
        if (j == size) {
            i += 1;
            j = i + 1;
        }
    }

    public record Combination<T>(T first, T second) {
    }
}

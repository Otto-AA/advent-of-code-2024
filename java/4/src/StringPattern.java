import java.util.*;

public class StringPattern implements Iterable<PatternChar> {
    private final Set<PatternChar> pattern = new HashSet<>();

    public StringPattern() {
    }

    public void addChar(char c, int rowOffset, int colOffset) {
        pattern.add(new PatternChar(c, rowOffset, colOffset));
    }

    @Override
    public Iterator<PatternChar> iterator() {
        return pattern.iterator();
    }
}

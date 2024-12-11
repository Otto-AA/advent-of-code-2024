import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

public class PlutoStonesEfficient {
    // map (stone, n) to the number of stones after n iterations
    private final Map<LongTuple, Long> cache = new ConcurrentHashMap<>(100000);

    public long countStonesAfterNIterations(long stone, int iterations) {
        // recursion exit condition
        if (iterations == 0) {
            return 1;
        }

        var key = new LongTuple(stone, iterations);
        if (!cache.containsKey(key)) {
            // recursion
            var result = applyRules(stone, iterations);
            cache.put(key, result);
        }
        return cache.get(key);
    }

    private long applyRules(long stone, int iterations) {
        if (stone == 0L) {
            return countStonesAfterNIterations(1L, iterations - 1);
        }
        var s = String.valueOf(stone);
        if (s.length() % 2 == 0) {
            return countStonesAfterNIterations(Long.parseLong(s.substring(0, s.length() / 2)), iterations - 1) + countStonesAfterNIterations(Long.parseLong(s.substring(s.length() / 2)), iterations - 1);
        }
        return countStonesAfterNIterations(stone * 2024L, iterations - 1);
    }
}

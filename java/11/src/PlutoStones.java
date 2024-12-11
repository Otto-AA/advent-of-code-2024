import java.util.ArrayList;
import java.util.List;

public class PlutoStones {
    private final static List<StoneRule> rules = List.of(
            new StoneRule(n -> n == 0L, n -> List.of(1L)),
            new StoneRule(PlutoStones::canSplitDigits, PlutoStones::splitDigits),
            new StoneRule(n -> true, n -> List.of(n * 2024L))
    );

    public static List<Long> applyRules(List<Long> stones) {
        var newStones = new ArrayList<Long>(stones.size() * 2);

        for (var stone : stones) {
            for (var rule : rules) {
                if (rule.canApply().test(stone)) {
                    newStones.addAll(rule.apply().apply(stone));
                    break;
                }
            }
        }

        return newStones;
    }

    private static boolean canSplitDigits(Long number) {
        return String.valueOf(number).length() % 2 == 0;
    }

    private static List<Long> splitDigits(Long number) {
        var s = String.valueOf(number);
        return List.of(
                Long.parseLong(s.substring(0, s.length() / 2)),
                Long.parseLong(s.substring(s.length() / 2))
        );
    }
}

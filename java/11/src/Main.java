import java.util.Arrays;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        var input = Arrays.stream(args[0].split(" ")).map(Long::parseLong).toList();
        System.out.println(applyRules(input, 25).size());
        System.out.println(applyRulesEfficient(input, 75));
    }

    private static List<Long> applyRules(List<Long> stones, int n) {
        var newStones = stones;
        for (int i = 0; i < n; i++) {
            newStones = PlutoStones.applyRules(newStones);
        }
        return newStones;
    }

    private static long applyRulesEfficient(List<Long> stones, int n) {
        var pluto = new PlutoStonesEfficient();

        return stones.parallelStream().mapToLong(stone -> pluto.countStonesAfterNIterations(stone, n)).sum();
    }
}
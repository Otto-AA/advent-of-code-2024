import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var input = parseInput();
        var map = new TopographicMap(input, input.size(), input.getFirst().size());
        System.out.println(partOne(map));
        System.out.println(partTwo(map));
    }

    public static final int BOTTOM = 0;
    public static final int TOP = 9;

    private static List<List<Integer>> parseInput() {
        var scanner = new Scanner(System.in);
        var map = new ArrayList<List<Integer>>();

        while (scanner.hasNextLine()) {
            map.add(new ArrayList<>());
            var list = map.getLast();
            var line = scanner.nextLine();
            for (int i = 0; i < line.length(); i++) {
                list.add(Integer.parseInt("" + line.charAt(i)));
            }
        }

        return map;
    }

    private static int partOne(TopographicMap map) {
        var searcher = new RouteSearcher(map, BOTTOM, TOP);
        searcher.computeReachability();
        // searcher.printReachabilityScores();

        return map.findByHeight(TOP).stream().mapToInt(p -> searcher.getUniqueReachableStartPoints(p).size()).sum();
    }

    private static int partTwo(TopographicMap map) {
        var searcher = new RouteSearcher(map, BOTTOM, TOP);
        searcher.computeReachability();

        return map.findByHeight(TOP).stream().mapToInt(p -> searcher.getReachableStartPoints(p).size()).sum();
    }
}
import java.util.LinkedList;
import java.util.Scanner;
import java.util.function.Function;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) {
        var lists = readInput();

        System.out.println(totalDistance(lists));
        System.out.println(similarity(lists));
    }

    private static LocationLists readInput() {
        var scanner = new Scanner(System.in);
        var locationLists = new LocationLists(new LinkedList<>(), new LinkedList<>());

        while (scanner.hasNext()) {
            var numbers = scanner.nextLine().split("\\s+");
            locationLists.left().add(Integer.parseInt(numbers[0]));
            locationLists.right().add(Integer.parseInt(numbers[1]));
        }

        return locationLists;
    }

    private static int totalDistance(LocationLists lists) {
        lists.left().sort((a, b) -> a - b);
        lists.right().sort((a, b) -> a - b);

        var distance = 0;
        for (int i = 0; i < lists.left().size(); i++) {
            distance += Math.abs(lists.left().get(i) - lists.right().get(i));
        }
        return distance;
    }

    private static int similarity(LocationLists lists) {
        var rightCounts = lists.right().stream().collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));

        return lists.left().stream().map(n -> n * rightCounts.getOrDefault(n, 0L)).mapToInt(Long::intValue).sum();
    }
}
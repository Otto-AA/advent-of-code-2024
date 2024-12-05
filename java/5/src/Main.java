import java.sql.Array;
import java.util.*;

public class Main {
    public static void main(String[] args) {
        var pageConstraints = new PageOrderingConstraints();
        var pageOrderings = new LinkedList<List<Integer>>();
        var readingConstraints = true;

        for (var line : readInput()) {
            if (line.isEmpty()) {
                readingConstraints = false;
            } else if (readingConstraints) {
                var numbers = line.split("\\|");
                var page = Integer.parseInt(numbers[0]);
                var pageAfterwards = Integer.parseInt(numbers[1]);
                pageConstraints.addConstraint(page, pageAfterwards);
            } else {
                var numbers = line.split(",");
                var parsedNumbers = new ArrayList<Integer>();
                for (var n : numbers) {
                    parsedNumbers.add(Integer.parseInt(n));
                }
                pageOrderings.add(parsedNumbers);
            }
        }

        var validOrderings = new LinkedList<List<Integer>>();
        var invalidOrderings = new LinkedList<List<Integer>>();

        for (var ordering : pageOrderings) {
            if (pageConstraints.isValidOrdering(ordering)) {
                validOrderings.add(ordering);
            } else {
                invalidOrderings.add(ordering);
            }
        }

        var sumOfMiddlePages = validOrderings.stream().mapToInt(o -> o.get(o.size() / 2)).sum();

        System.out.println("Sum of middle pages: " + sumOfMiddlePages);

        var fixedOrderings = invalidOrderings.stream().map(pageConstraints::fixOrdering).toList();
        var sumOfMiddlePagesFixed = fixedOrderings.stream().mapToInt(o -> o.get(o.size() / 2)).sum();

        System.out.println("Sum of middle pages fixed: " + sumOfMiddlePagesFixed);

        for (var ordering : fixedOrderings) {
            if (!pageConstraints.isValidOrdering(ordering))
                System.out.println(ordering);
        }
    }

    private static ArrayList<String> readInput() {
        var in = new Scanner(System.in);
        var lines = new ArrayList<String>();

        while (in.hasNextLine()) {
            lines.add(in.nextLine());
        }

        return lines;
    }
}
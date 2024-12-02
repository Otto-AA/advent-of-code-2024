import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var reports = readInput();

        var safeReports = reports.stream().filter(Main::isSafe).count();
        System.out.println("Safe reports: " + safeReports);

        var safeReportsWithDamper = reports.stream().filter(report -> {
            for (int i = 0; i < report.length; i++) {
                if (isSafe(report, i)) {
                    return true;
                }
            }
            return false;
        }).count();

        System.out.println("Safe reports with damper: " + safeReportsWithDamper);
    }

    private static List<int[]> readInput() {
        var scanner = new Scanner(System.in);
        var input = new LinkedList<int[]>();

        while (scanner.hasNext()) {
            var numbers = scanner.nextLine().split("\\s+");
            input.add(Arrays.stream(numbers).mapToInt(Integer::parseInt).toArray());
        }

        return input;
    }

    private static boolean isSafe(int[] report) {
        return isSafe(report, -1);
    }

    private static boolean isSafe(int[] report, int ignoredLevel) {
        Integer prev = null;
        Boolean direction = null;
        for (int i = 0; i < report.length; i++) {
            if (i == ignoredLevel) {
                continue;
            }
            var level = report[i];

            if (prev != null) {
                if (direction == null) {
                    direction = prev < level;
                }
                if (direction != prev < level) {
                    return false;
                }
                if (prev == level || Math.abs(prev - level) > 3) {
                    return false;
                }
            }
            prev = level;
        }
        return true;
    }
}
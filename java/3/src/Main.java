import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.Scanner;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) {
        var input = readInput();
        var sum = calculateMultiplications(input);
        var sumWithConds = calculateMultiplicationsWithConditions(input);
        System.out.println("Multiplication sum: " + sum);
        System.out.println("Multiplication sum (conditional): " + sumWithConds);
    }

    private static String readInput() {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        return reader.lines().collect(Collectors.joining("\n"));
    }

    private static int calculateMultiplications(String input) {
        var pattern = Pattern.compile("mul\\((\\d{1,3}),(\\d{1,3})\\)");

        return pattern.matcher(input).results()
                .mapToInt(m -> Integer.parseInt(m.group(1)) * Integer.parseInt(m.group(2)))
                .sum();
    }

    private static int calculateMultiplicationsWithConditions(String input) {
        // foo do don't mul(1,2) do mul(1,2) don't mul(1,2)
        var scanner = new Scanner(input).useDelimiter("do\\(\\)");

        int sum = 0;
        var mulPattern = Pattern.compile("mul\\((\\d{1,3}),(\\d{1,3})\\)");

        while (scanner.hasNext()) {
            var withinDos = scanner.next();
            var todo = withinDos.split("don't\\(\\)")[0];
            var matcher = mulPattern.matcher(todo);
            while (matcher.find()) {
                var x = matcher.group(1);
                var y = matcher.group(2);
                sum += Integer.parseInt(x) * Integer.parseInt(y);
            }
        }

        return sum;
    }
}
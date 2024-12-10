import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var calculations = readInput();
        var solver = new Solver(List.of(
                Long::sum,
                (a, b) -> a * b
        ));
        var solverTwo = new Solver(List.of(
                Long::sum,
                (a, b) -> a * b,
                (a, b) -> Long.parseLong("" + a + b)
        ));

        System.out.println(calibrate(calculations, solver));
        System.out.println(calibrate(calculations, solverTwo));
    }

    private static List<Calculation> readInput() {
        var scanner = new Scanner(System.in);
        var calculations = new ArrayList<Calculation>();

        while (scanner.hasNextLine()) {
            var line = scanner.nextLine();
            var parts = line.split(": ");
            var total = Long.parseLong(parts[0]);
            var operands = Arrays.stream(parts[1].split(" ")).map(Long::parseLong).toList();
            calculations.add(new Calculation(total, operands));
        }

        return calculations;
    }

    private static long calibrate(List<Calculation> calculations, Solver solver) {
        return calculations.stream()
                .filter(solver::isSolvable)
                .mapToLong(Calculation::expectedTotal)
                .sum();
    }
}
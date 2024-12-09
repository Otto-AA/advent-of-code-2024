import java.util.*;

public class Main {
    public static void main(String[] args) {
        var lines = readInput();
        var antennas = parseAntennas(lines);
        int rows = lines.size();
        int cols = lines.getFirst().length();

        var antinodes = findAntinodes(antennas, rows, cols);
        System.out.println(antinodes.size());

        antinodes = findAntinodesWithHarmonics(antennas, rows, cols);
        System.out.println(antinodes.size());
    }

    private static List<String> readInput() {
        var scanner = new Scanner(System.in);
        var lines = new ArrayList<String>();

        while (scanner.hasNextLine()) {
            lines.add(scanner.nextLine());
        }

        return lines;
    }

    private static Antennas parseAntennas(List<String> lines) {
        var antennas = new Antennas();

        int row = 0;

        for (var line : lines) {
            for (int col = 0; col < line.length(); col++) {
                var c = line.charAt(col);
                if ((c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')) {
                    antennas.add(c, row, col);
                }
            }
            row += 1;
        }

        return antennas;
    }

    private static Set<Point> findAntinodes(Antennas antennas, int rows, int cols) {
        var antinodes = new HashSet<Point>();

        for (var type : antennas.iterateTypes()) {
            for (var combination : antennas.iterateLocationTuples(type)) {
                var a = combination.first();
                var b = combination.second();
                var vector = new Vector(a, b);
                var candidateA = a.subtract(vector);
                var candidateB = b.add(vector);
                if (candidateA.row() >= 0 && candidateA.row() < rows && candidateA.col() >= 0 && candidateA.col() < cols) {
                    antinodes.add(candidateA);
                }
                if (candidateB.row() >= 0 && candidateB.row() < rows && candidateB.col() >= 0 && candidateB.col() < cols) {
                    antinodes.add(candidateB);
                }
            }
        }

        return antinodes;
    }

    private static Set<Point> findAntinodesWithHarmonics(Antennas antennas, int rows, int cols) {
        var antinodes = new HashSet<Point>();

        for (var type : antennas.iterateTypes()) {
            for (var combination : antennas.iterateLocationTuples(type)) {
                var a = combination.first();
                var vector = new Vector(a, combination.second());

                var candidate = a;

                while (candidate.row() >= 0 && candidate.row() < rows && candidate.col() >= 0 && candidate.col() < cols) {
                    antinodes.add(candidate);
                    candidate = candidate.subtract(vector);
                }

                candidate = a.add(vector);
                while (candidate.row() >= 0 && candidate.row() < rows && candidate.col() >= 0 && candidate.col() < cols) {
                    antinodes.add(candidate);
                    candidate = candidate.add(vector);
                }
            }
        }

        return antinodes;
    }
}
import java.util.*;

public class Main {
    public static void main(String[] args) {
        var puzzle = readInput();
        var patternMatcher = new PatternMatcher(new StringTable(puzzle));

        System.out.println(countUniqueMatches(patternMatcher, buildXMASPatterns()));
        System.out.println(countUniqueMatches(patternMatcher, buildMasCrossPatterns()));
    }

    private static <T> int countUniqueMatches(PatternMatcher patternMatcher, List<StringPattern> patterns) {
        int matches = 0;
        for (var pattern : patterns) {
            for (var matchingCell : patternMatcher.findMatches(pattern)) {
                matches++;
            }
        }
        return matches;
    }

    private static ArrayList<String> readInput() {
        var in = new Scanner(System.in);
        var lines = new ArrayList<String>();

        while (in.hasNextLine()) {
            lines.add(in.nextLine());
        }

        return lines;
    }

    private static List<StringPattern> buildXMASPatterns() {
        var patterns = new LinkedList<StringPattern>();

        for (var direction : buildAllDirections()) {
            var pattern = new StringPattern();
            addStringToPattern(pattern, "XMAS", direction);
            patterns.add(pattern);
        }

        return patterns;
    }

    private static List<StringPattern> buildMasCrossPatterns() {
        var patterns = new LinkedList<StringPattern>();
        for (var mainDirection : buildDiagonalDirections()) {
            var crossDirection = buildCrossDirection(mainDirection);

            // one step in main direction, one step back in other direction
            var offset = new Direction(
                    mainDirection.row() - crossDirection.row(),
                    mainDirection.col() - crossDirection.col()
            );
            var pattern = new StringPattern();
            addStringToPattern(pattern, "MAS", mainDirection);
            addStringToPattern(pattern, "MAS", crossDirection, offset);

            patterns.add(pattern);

        }

        return patterns;
    }

    private static List<Direction> buildAllDirections() {
        var directions = new LinkedList<Direction>();

        for (int row : List.of(-1, 0, 1)) {
            for (int col : List.of(-1, 0, 1)) {
                if (row == 0 && col == 0) {
                    continue;
                }
                directions.add(new Direction(row, col));
            }
        }

        return directions;
    }

    private static List<Direction> buildDiagonalDirections() {
        var directions = new LinkedList<Direction>();

        for (int row : List.of(-1, 1)) {
            for (int col : List.of(-1, 1)) {
                directions.add(new Direction(row, col));
            }
        }

        return directions;
    }

    private static Direction buildCrossDirection(Direction direction) {
        // rotate 90° clockwise
        return new Direction(-direction.col(), direction.row());
    }

    private static void addStringToPattern(StringPattern pattern, String string, Direction direction) {
        addStringToPattern(pattern, string, direction, new Direction(0, 0));
    }

    private static void addStringToPattern(StringPattern pattern, String string, Direction direction, Direction offset) {
        for (int nthChar = 0; nthChar < string.length(); nthChar++) {
            pattern.addChar(string.charAt(nthChar), offset.row() + nthChar * direction.row(), offset.col() + nthChar * direction.col());
        }
    }

}
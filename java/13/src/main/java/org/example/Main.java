package org.example;

import java.util.*;
import java.util.List;
import java.util.regex.Pattern;

public class Main {
    public static void main(String[] args) {
        var scanner = new Scanner(System.in);
        scanner.useDelimiter("\n");

        var machines = parse(scanner);

        System.out.println(partOne(machines));
        System.out.println(partTwo(machines));
    }

    public static List<ClawMachineConfig> parse(Iterator<String> input) {
        var machines = new ArrayList<ClawMachineConfig>();

        while (input.hasNext()) {
            var buttonA = parseButton(input.next());
            var buttonB = parseButton(input.next());
            var target = parsePoint(input.next());
            machines.add(new ClawMachineConfig(buttonA, buttonB, target));

            if (input.hasNext()) input.next();
        }

        return machines;
    }

    private static ClawMachineButton parseButton(String input) {
        var point = parsePoint(input);
        return new ClawMachineButton(point.x(), point.y());
    }

    private static Point parsePoint(String input) {
        var matcherX = Pattern.compile("X.(\\d+)").matcher(input);
        var matcherY = Pattern.compile("Y.(\\d+)").matcher(input);
        matcherX.find();
        matcherY.find();

        return new Point(Long.parseLong(matcherX.group(1)), Long.parseLong(matcherY.group(1)));
    }


    public static long partOne(List<ClawMachineConfig> machines) {
        return machines.stream().map(Solver::solve).filter(Objects::nonNull).mapToLong(s -> s.a() * 3L + s.b()).sum();
    }

    public static long partTwo(List<ClawMachineConfig> machines) {
        var newMachines = machines.stream().map(Main::unitConversion).toList();
        return partOne(newMachines);
    }

    public static ClawMachineConfig unitConversion(ClawMachineConfig machine) {
        return new ClawMachineConfig(machine.A(), machine.B(), new Point(machine.target().x() + 10000000000000L, machine.target().y() + 10000000000000L));
    }
}
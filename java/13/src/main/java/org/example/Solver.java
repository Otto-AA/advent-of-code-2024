package org.example;

public class Solver {
    public static Solution solve(ClawMachineConfig machine) {
        double slopeA = (double) machine.A().y() / machine.A().x();
        double slopeB = (double) machine.B().y() / machine.B().x();
        double offsetB = offset(machine.target(), slopeB);

        double crossingPointX = machine.target().x();
        if (slopeA != slopeB) {
            crossingPointX = offsetB / (slopeA - slopeB);
        } else if (machine.A().x() > machine.B().x()) {
            crossingPointX = 0D;
        }

        long a = Math.round(crossingPointX / machine.A().x());
        long b = Math.round((machine.target().x() - crossingPointX) / machine.B().x());

        var solution = new Solution(a, b);

        if (check(machine, solution)) {
            return solution;
        }
        return null;
    }

    // offset at x = 0, s.t. a line with this slope reaches the target
    // i.e. target.y = offset + target.x*slope
    private static double offset(Point target, double slope) {
        return target.y() - target.x() * slope;
    }

    private static boolean check(ClawMachineConfig machine, Solution solution) {
        return solution.a() * machine.A().x() + solution.b() * machine.B().x() == machine.target().x() && solution.a() * machine.A().y() + solution.b() * machine.B().y() == machine.target().y();
    }
}

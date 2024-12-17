package org.example;

import static org.junit.jupiter.api.Assertions.*;
import org.junit.jupiter.api.Test;

public class SolverTests {
    @Test
    void onlyButtonA() {
        var machine = new ClawMachineConfig(new ClawMachineButton(10, 10), new ClawMachineButton(100, 100), new Point(30, 30));

        var solution = Solver.solve(machine);

        assertEquals(new Solution(3, 0), solution);
    }

    @Test
    void onlyButtonB() {
        var machine = new ClawMachineConfig(new ClawMachineButton(100, 100), new ClawMachineButton(10, 10), new Point(30, 30));

        var solution = Solver.solve(machine);

        assertEquals(new Solution(0, 3), solution);
    }

    @Test
    void bothButtons() {
        var machine = new ClawMachineConfig(new ClawMachineButton(94, 34), new ClawMachineButton(22, 67), new Point(8400, 5400));

        var solution = Solver.solve(machine);

        assertEquals(new Solution(80, 40), solution);
    }

    @Test
    void noSolution() {
        var machine = new ClawMachineConfig(new ClawMachineButton(10, 20), new ClawMachineButton(5, 30), new Point(123, 456));

        var solution = Solver.solve(machine);

        assertNull(solution);
    }
}

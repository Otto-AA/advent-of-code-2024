package org.example;

import static org.junit.jupiter.api.Assertions.*;
import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.List;

public class MainTests {
    private static List<ClawMachineConfig> parse(String input) {
        var inputIterator = Arrays.stream(input.split("\\n")).iterator();
        return Main.parse(inputIterator);
    }
    private static final List<ClawMachineConfig> sample = parse("""
                Button A: X+94, Y+34
                Button B: X+22, Y+67
                Prize: X=8400, Y=5400
                
                Button A: X+26, Y+66
                Button B: X+67, Y+21
                Prize: X=12748, Y=12176
                
                Button A: X+17, Y+86
                Button B: X+84, Y+37
                Prize: X=7870, Y=6450
                
                Button A: X+69, Y+23
                Button B: X+27, Y+71
                Prize: X=18641, Y=10279""");

    @Test
    void partOneSample() {
        assertEquals(480L, Main.partOne(sample));
    }

    @Test
    void partTwoSample() {
        assertEquals(875318608908L, Main.partTwo(sample));
    }
}

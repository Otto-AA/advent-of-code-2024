package org.example;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

public class MainTests {
    private static List<Region<Character>> parse(String area) {
        var chars = new ArrayList<List<Character>>();

        area.lines().forEach(line -> chars.add(line.chars().mapToObj(c -> (char) c).toList()));

        return RegionsBuilder.from(chars).build();
    }

    private static final List<Region<Character>> sample = parse("""
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE""");

    @Test
    void partOneSample() {
        assertEquals(1930, Main.partOne(sample));
    }

    @Test
    void partTwoSample() {
        assertEquals(1206, Main.partTwo(sample));
    }
}

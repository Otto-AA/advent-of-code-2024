package org.example;

import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var map = readInput();
        var regions = RegionsBuilder.from(map).build();

        System.out.println(partOne(regions));
        System.out.println(partTwo(regions));
    }

    private static List<List<Character>> readInput() {
        var scanner = new Scanner(System.in);
        var result = new ArrayList<List<Character>>();

        while (scanner.hasNextLine()) {
            List<Character> chars = scanner.nextLine().chars().mapToObj(c -> (char) c).toList();
            result.add(chars);
        }

        return result;
    }

    public static <T> long partOne(List<Region<T>> regions) {
        return regions.stream().mapToLong(Main::partOne).sum();
    }

    public static <T> long partOne(Region<T> region) {
        return region.calculatePerimeter() * region.area();
    }

    public static <T> long partTwo(List<Region<T>> regions) {
        return regions.stream().mapToLong(Main::partTwo).sum();
    }

    public static <T> long partTwo(Region<T> region) {
        return region.countSides() * region.area();
    }
}
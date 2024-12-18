package org.example;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Simulation {
    private final int height;
    private final int width;
    private final List<Robot> robots;

    public Simulation(int height, int width, List<Robot> robots) {
        this.height = height;
        this.width = width;
        this.robots = robots;
    }

    public void fastForward(int seconds) {
        robots.replaceAll(robot -> fastForward(robot, seconds));
    }

    public void rewind(int seconds) {
        robots.replaceAll(robot -> fastForward(robot, -seconds));
    }

    private Robot fastForward(Robot robot, int seconds) {
        return new Robot(new Point(posMod(robot.position().x() + seconds * robot.velocity().x(), width), posMod(robot.position().y() + seconds * robot.velocity().y(), height)), robot.velocity());
    }

    private static int posMod(int n, int mod) {
        return (n % mod + mod) % mod;
    }

    public int width() {
        return width;
    }

    public int height() {
        return height;
    }

    public List<Robot> getRobots() {
        return robots;
    }

    public List<List<Integer>> currentMap() {
        var map = new ArrayList<List<Integer>>(height);

        // Initialize to matrix of 0s
        for (int row = 0; row < height; row++) {
            var rowList = new ArrayList<Integer>(width);
            rowList.addAll(Collections.nCopies(width, 0));
            map.add(rowList);
        }

        for (var robot : robots) {
            var x = robot.position().x();
            var y = robot.position().y();
            map.get(y).set(x, 1 + map.get(y).get(x));
        }

        return map;
    }
}

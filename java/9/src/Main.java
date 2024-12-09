import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var blocks = readInput();

        System.out.println(blocks.size());
        System.out.println("\nChecksum: " + computeRearrangedChecksum(blocks));
        System.out.println("\nChecksum: " + computeWholeFileRearrangementChecksum(blocks));
    }

    private static List<Block> readInput() {
        var scanner = new Scanner(System.in);
        var line = scanner.nextLine();
        var blocks = new ArrayList<Block>();

        for (int i = 0; i < line.length(); i += 2) {
            var fileBlocks = Character.getNumericValue(line.charAt(i));
            var freeBlocks = 0;
            if (i + 1 < line.length()) {
                freeBlocks = Character.getNumericValue(line.charAt(i + 1));
            }
            blocks.add(new Block(fileBlocks, freeBlocks));
        }

        return blocks;
    }

    private static long computeRearrangedChecksum(List<Block> blocks) {
        /*
        Keep track of:
        - current file block
        - current last file block (and how much of it was used to fill up at the start)

        Increase the current file block and then fill up the free space of it
        with blocks from the current last file block.
         */
        var checkSum = 0L;
        var positionCounter = 0;
        var currentLastBlockIndex = blocks.size() - 1;
        var currentLastBlock = blocks.getLast();
        var usedFromLastBlock = 0;

        for (int i = 0; i < currentLastBlockIndex; i++) {
            var block = blocks.get(i);
            for (int j = 0; j < block.fileBlocks(); j++) {
                System.out.printf("[%d]", i);
                checkSum += (long) positionCounter * i;
                positionCounter++;
            }

            var freeSpace = block.freeBlocks();
            while (freeSpace > 0) {
                if (usedFromLastBlock >= currentLastBlock.fileBlocks()) {
                    currentLastBlockIndex -= 1;
                    currentLastBlock = blocks.get(currentLastBlockIndex);
                    usedFromLastBlock = 0;
                }
                if (currentLastBlockIndex == i) {
                    break;
                }
                System.out.printf("(%d)", currentLastBlockIndex);
                checkSum += (long) positionCounter * currentLastBlockIndex;
                positionCounter++;
                usedFromLastBlock++;
                freeSpace--;
            }
        }

        for (int j = 0; j < currentLastBlock.fileBlocks() - usedFromLastBlock; j++) {
            System.out.printf("<%d>", currentLastBlockIndex);
            checkSum += (long) positionCounter * currentLastBlockIndex;
            positionCounter++;
        }

        return checkSum;
    }

    private static long computeWholeFileRearrangementChecksum(List<Block> blocks) {
        /*
        Keep track of:
        - current file block
        - set of used file blocks (both, normally from the start, and moved from the end to the start)

        Increase the current file block and then fill up the free space of it
        with files from the end.
         */
        var checkSum = 0L;
        var positionCounter = 0;
        var usedFiles = HashSet.<Integer>newHashSet(blocks.size());

        for (int i = 0; i < blocks.size(); i++) {
            var block = blocks.get(i);
            if (usedFiles.contains(i)) {
                System.out.print(".".repeat(block.fileBlocks()));
                positionCounter += block.fileBlocks();
            } else {
                for (int j = 0; j < block.fileBlocks(); j++) {
                    System.out.printf("[%d]", i);
                    checkSum += (long) positionCounter * i;
                    positionCounter++;
                }
                usedFiles.add(i);
            }

            var freeSpace = block.freeBlocks();

            for (int j = blocks.size() - 1; j > i && freeSpace > 0; j--) {
                if (usedFiles.contains(j)) {
                    continue;
                }
                var movedBlock = blocks.get(j);
                if (movedBlock.fileBlocks() <= freeSpace) {
                    freeSpace -= movedBlock.fileBlocks();
                    usedFiles.add(j);
                    for (int x = 0; x < movedBlock.fileBlocks(); x++) {
                        System.out.printf("(%d)", j);
                        checkSum += (long) positionCounter * j;
                        positionCounter++;
                    }
                }
            }

            System.out.print(".".repeat(freeSpace));
            positionCounter += freeSpace;
        }

        return checkSum;
    }
}
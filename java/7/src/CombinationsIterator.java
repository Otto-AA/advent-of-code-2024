import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

public class CombinationsIterator<T> implements Iterator<List<T>> {
    private final List<T> elements;
    private final int[] currentCombination;
    private boolean hasRemainingCombinations = true;

    public CombinationsIterator(List<T> elements, int size) {
        this.elements = elements;
        currentCombination = new int[size];
    }

    @Override
    public boolean hasNext() {
        return hasRemainingCombinations;
    }

    @Override
    public List<T> next() {
        var result = mapCombinationToElements();
        goToNextCombination();
        return result;
    }

    private List<T> mapCombinationToElements() {
        return Arrays.stream(currentCombination).mapToObj(elements::get).toList();
    }

    private void goToNextCombination() {
        // increase starting from the last element
        // if last number is already the last element
        // then reset it to 0 and continue with
        // the previous element
        var i = currentCombination.length - 1;
        var hasResetted = true;

        while (hasResetted && i >= 0) {
            currentCombination[i] = (currentCombination[i] + 1) % elements.size();
            if (currentCombination[i] == 0) {
                i--;
            } else {
                hasResetted = false;
            }
        }
        // resetted all elements
        if (i < 0) {
            hasRemainingCombinations = false;
        }
    }
}

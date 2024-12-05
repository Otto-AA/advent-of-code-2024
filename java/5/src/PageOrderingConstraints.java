import java.util.*;

public class PageOrderingConstraints {
    private final Map<Integer, Set<Integer>> mustBeBefore = new HashMap<>();
    private final Map<Integer, Set<Integer>> mustBeAfter = new HashMap<>();

    public void addConstraint(int page, int pageAfterwards) {
        mustBeBefore.putIfAbsent(page, new HashSet<>());
        mustBeBefore.get(page).add(pageAfterwards);
        mustBeAfter.putIfAbsent(pageAfterwards, new HashSet<>());
        mustBeAfter.get(pageAfterwards).add(page);
    }

    public boolean isValidOrdering(List<Integer> pages) {
        var earlierPages = new HashSet<>(pages);
        for (var page : pages.reversed()) {
            earlierPages.remove(page);
            var pagesThatMustOccurLater = mustBeBefore.getOrDefault(page, new HashSet<>());

            var intersection = new HashSet<>(earlierPages);
            intersection.retainAll(pagesThatMustOccurLater);

            if (!intersection.isEmpty()) {
                return false;
            }
        }
        return true;
    }

    public List<Integer> fixOrdering(List<Integer> pages) {
        // Iteratively add a number which is currently allowed.
        // A number is allowed, if no other number must be before it
        var ordering = new ArrayList<Integer>();
        var remaining = new HashSet<>(pages);

        while (ordering.size() < pages.size()) {
            var prevSize = ordering.size();
            for (var page : remaining) {
                // check if the page must be after a page that is not yet used
                var x = mustBeAfter.getOrDefault(page, new HashSet<>());
                var intersection = new HashSet<>(x);
                intersection.retainAll(remaining);
                if (intersection.isEmpty()) {
                    ordering.add(page);
                    remaining.remove(page);
                    break;
                }
            }
            if (prevSize == ordering.size()) {
                throw new RuntimeException("Could not find a valid ordering for " + pages);
            }
        }

        return ordering;
    }
}

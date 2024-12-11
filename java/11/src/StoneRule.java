import java.util.List;
import java.util.function.Function;
import java.util.function.Predicate;

public record StoneRule(Predicate<Long> canApply, Function<Long, List<Long>> apply) {
}

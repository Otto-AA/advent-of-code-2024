import java.util.List;
import java.util.function.LongBinaryOperator;

public class Solver {
    private final List<LongBinaryOperator> operators;

    public Solver(List<LongBinaryOperator> operators) {
        this.operators = operators;
    }

    public boolean isSolvable(Calculation calculation) {
        var operands = calculation.operands();
        var iterator = new CombinationsIterator<>(operators, operands.size() - 1);

        while (iterator.hasNext()) {
            var operatorsCombination = iterator.next();
            var result = applyOperators(calculation.operands(), operatorsCombination);
            if (result == calculation.expectedTotal()) {
                return true;
            }
        }

        return false;
    }

    private static long applyOperators(List<Long> operands, List<LongBinaryOperator> operators) {
        long result = operands.getFirst();
        for (int i = 0; i < operators.size(); i++) {
            var op = operators.get(i);
            var operand = operands.get(i + 1);
            result = op.applyAsLong(result, operand);
        }

        return result;
    }
}

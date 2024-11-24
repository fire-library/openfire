import { CalculationComponent, Parameter } from "src/bindings";
import Equation from "./Equation";
import EquationWithResult from "./EquationWithResult";
import Text from "./Text";

export default function ViewFactors({
  component,
  parameter,
}: {
  component: CalculationComponent;
  parameter: Parameter;
}) {
  if ("Text" in component) {
    return <Text text={component.Text} />;
  }
  if ("Equation" in component) {
    return <Equation equation={component.Equation} />;
  }
  if ("EquationWithResult" in component) {
    return (
      <EquationWithResult
        equation={component.EquationWithResult}
        parameter={parameter}
      />
    );
  }
  return null;
}

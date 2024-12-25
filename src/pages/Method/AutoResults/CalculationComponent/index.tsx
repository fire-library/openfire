import { CalculationComponent, Parameter, ParameterType } from "src/bindings";
import Equation from "./Equation";
import EquationWithResult from "./EquationWithResult";
import Text from "./Text";
import { stringedParam } from "src/bindingHelpers";

export default function ViewFactors({
  component,
  parameter,
}: {
  component: CalculationComponent;
  parameter: ParameterType;
}) {
  if ("Text" in component) {
    return <Text text={component.Text} />;
  }
  if ("Equation" in component) {
    return <Equation equation={component.Equation} />;
  }
  if ("EquationWithResult" in component) {
    const p = stringedParam(parameter);
    if (p == null) {
      return null;
    }
    return (
      <EquationWithResult
        equation={component.EquationWithResult}
        parameter={p}
      />
    );
  }
  return null;
}

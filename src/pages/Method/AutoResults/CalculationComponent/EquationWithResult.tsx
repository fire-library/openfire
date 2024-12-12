import { InlineMath } from "react-katex";
import { Parameter } from "src/bindings";
import { parameterValue } from "src/pages/components/ParameterValue";

export default function ViewFactors({
  equation,
  parameter,
}: {
  equation: string;
  parameter: Parameter;
}) {
  return (
    <div className="flex">
      <InlineMath>{`${equation} = ${parameterValue(parameter)} \\space ${parameter.units || ""}`}</InlineMath>
    </div>
  );
}

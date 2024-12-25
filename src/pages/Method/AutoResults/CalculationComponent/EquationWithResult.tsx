import { InlineMath } from "react-katex";
import { Parameter, ParameterType } from "src/bindings";

export default function ViewFactors({
  equation,
  parameter,
}: {
  equation: string;
  parameter: Parameter<string>;
}) {
  return (
    <div className="flex">
      <InlineMath>{`${equation} = ${parameter?.value} \\space ${parameter.units || ""}`}</InlineMath>
    </div>
  );
}

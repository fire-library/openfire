import { InlineMath } from "react-katex";
import { Parameter } from "src/bindings";
import { getResultValue } from "src/pages/Method/helpers";

export default function ViewFactors({
  equation,
  parameter,
}: {
  equation: string;
  parameter: Parameter;
}) {
  return (
    <div className="flex">
      <InlineMath>{`${equation} = ${getResultValue(parameter.value)}${parameter.units}`}</InlineMath>
    </div>
  );
}

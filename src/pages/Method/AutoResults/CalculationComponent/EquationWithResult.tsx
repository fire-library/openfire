import { InlineMath } from "react-katex";
import { Parameter } from "src/bindings";

export default function ViewFactors({
  equation,
  parameter,
}: {
  equation: string;
  parameter: Parameter;
}) {
  return (
    <div className="flex">
      <InlineMath>{`${equation} = ${parameter.value} \\space ${parameter.units}`}</InlineMath>
    </div>
  );
}

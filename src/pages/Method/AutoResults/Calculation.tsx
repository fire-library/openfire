import { useEffect, useState } from "react";
import { commands, Parameter, CalculationComponent } from "src/bindings";
import CalcComponent from "./CalculationComponent";

export default function ViewFactors({
  parameter,
  type,
}: {
  parameter: Parameter;
  type: "numbers" | "symbols";
}) {
  const [components, setComponents] = useState<CalculationComponent[][]>([[]]);

  useEffect(() => {
    if (type == "symbols") {
      commands.getEquationWithSymbols(parameter).then((equation) => {
        if (equation.status == "ok") {
          setComponents(equation.data);
        }
      });
    }
    if (type == "numbers") {
      commands.getEquationWithNumbers(parameter).then((equation) => {
        if (equation.status == "ok") {
          setComponents(equation.data);
        } else {
          setComponents([[]]);
        }
      });
    }
  }, [parameter, type]);

  return (
    <div className="flex flex-col gap-5">
      {components.map((row, i) => (
        <div key={i} className="flex flex-row items-center gap-10">
          {row.map((col, j) => (
            <CalcComponent key={j} component={col} parameter={parameter} />
          ))}
        </div>
      ))}
    </div>
  );
}

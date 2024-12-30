import { Card, CardHeader, CardBody } from "src/components";
import { Calculation, Parameter, ParameterType } from "src/bindings";
import { stringedParam } from "src/bindingHelpers";
import { InlineMath } from "react-katex";

export function QuickValue({ param }: { param: ParameterType }) {
  const p = stringedParam(param);
  if (p?.value == null) {
    return null;
  }

  return (
    <InlineMath>
      {`${p?.id} = ${p?.value} \\space
        ${p?.units || ""}`}
    </InlineMath>
  );
}

export default function MaterialInput({ results }: { results: Calculation }) {
  return (
    <>
      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                Results
              </h3>
            </div>
          </div>
        </CardHeader>
        <CardBody>
          <div className="flex flex-col gap-3">
            {results.stale
              ? "Invalid input"
              : results.steps.map((step) => {
                  {
                    return step.parameters.map((parameter) => {
                      {
                        const p = stringedParam(parameter);
                        return (
                          (p?.value !== null && (
                            <QuickValue param={parameter} />
                          )) ||
                          null
                        );
                      }
                    });
                  }
                })}
          </div>
        </CardBody>
      </Card>
    </>
  );
}

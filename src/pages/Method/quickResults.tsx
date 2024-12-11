import { Card, CardHeader, CardBody } from "src/components";
import { Calculation, Parameter } from "src/bindings";
import { InlineMath } from "react-katex";

export function QuickValue({ param }: { param: Parameter }) {
  if (param.value == null) {
    return null;
  }
  return (
    <InlineMath>
      {`${param.id} = ${param.value} \\space
        ${param.units || ""}`}
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
                Quick Results
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
                        return (
                          (parameter.value !== null && (
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

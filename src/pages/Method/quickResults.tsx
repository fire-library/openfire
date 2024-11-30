import { Card, CardHeader, CardBody } from "src/components";
import { Calculation } from "src/bindings";
import { InlineMath } from "react-katex";

export default function MaterialInput({ results }: { results: Calculation }) {
  const displayParam = results.steps[0].parameters[0];
  const resultValue = displayParam.value;

  const Equation = () => {
    if (resultValue == null) {
      return "Not yet calculated";
    }
    if (results.stale) {
      return "Invalid Input";
    }
    return (
      <InlineMath>
        {`${displayParam.id} = ${displayParam.value}
        ${displayParam.units || ""}`}
      </InlineMath>
    );
  };

  return (
    <>
      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                {displayParam.name}
              </h3>
            </div>
          </div>
        </CardHeader>
        <CardBody>
          <div className="flex flex-col gap-3">
            <Equation />
          </div>
        </CardBody>
      </Card>
    </>
  );
}

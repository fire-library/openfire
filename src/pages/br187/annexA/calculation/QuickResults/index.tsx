import { Card, CardHeader, CardBody } from "src/components";
import { Br187BasicOutput } from "src/bindings";

export default function MaterialInput({
  results,
}: {
  results: Br187BasicOutput | null;
}) {
  return (
    <>
      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                Heat Flux at Receiver
              </h3>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              The heat flux at the receiver. Run the calculation to generate the
              calculation sheet.
            </p>
          </div>
        </CardHeader>
        <CardBody>
          {results
            ? `${results.calculation.incidentHeatFlux} KW/m2`
            : "Invalid input"}
        </CardBody>
      </Card>
    </>
  );
}

import { ParametricFireInput } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import { Item } from "./FireloadCard";

export default function Results({ input }: { input: ParametricFireInput }) {
  return (
    <Card>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Additional Paramters
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            Additional required parameters specific to the calculation.
          </p>
        </div>
      </CardHeader>
      <CardBody>
        <ul className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 py-4 w-full gap-2">
          <Item>Timestep: {input.timestep} s</Item>
        </ul>
      </CardBody>
    </Card>
  );
}

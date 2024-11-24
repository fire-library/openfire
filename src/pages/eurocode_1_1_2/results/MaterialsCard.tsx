import { MaterialInput } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import Table, { TH, TD } from "src/components/form/Table";

export default function Results({ materials }: { materials: MaterialInput[] }) {
  return (
    <Card>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Materials
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            The materials provided for the calculation.
          </p>
        </div>
      </CardHeader>
      <CardBody>
        <Table>
          <thead>
            <tr className="mt-4">
              <TH>Name</TH>
              <TH>
                Density (kg/m<sup>3</sup>)
              </TH>
              <TH>Specific Heat (J/kgK)</TH>
              <TH>Thermal Conductivity (W/mK)</TH>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-300">
            {materials.map((material, index) => (
              <tr key={index}>
                <TD className="max-w-lg">{material?.name}</TD>
                <TD>{material?.density}</TD>
                <TD>{material?.specificHeat}</TD>
                <TD>{material?.thermalConductivity}</TD>
              </tr>
            ))}
          </tbody>
        </Table>
      </CardBody>
    </Card>
  );
}

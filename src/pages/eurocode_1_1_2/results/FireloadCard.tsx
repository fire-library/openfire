import { FireLoadInput, FireloadFactors } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import Table, { TH, TD } from "src/components/form/Table";

export default function Results({
  fireloadInput,
  fireloadFactors,
}: {
  fireloadInput: FireLoadInput;
  fireloadFactors: FireloadFactors;
}) {
  return (
    <Card>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Fire Load
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            The fire load characteristics of the compartment.
          </p>
        </div>
      </CardHeader>
      <CardBody>
        <ul className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 py-4 w-full gap-2">
          <Item>
            Characteristic Fireload Desnsity:{" "}
            {fireloadInput.characteristicFireLoadDensity}
          </Item>
          <Item>
            Fire Growth Rate: {fireloadInput.growthRate?.toUpperCase()}
          </Item>
          <Item>Combustion Factor: {fireloadInput.combustionFactor}</Item>
          <Item>
            Risk of Activation due to Occupancy Type: {fireloadInput.delta2}
          </Item>
        </ul>
      </CardBody>
      <CardBody>
        <Table>
          <thead>
            <tr className="mt-4">
              <TH>Category</TH>
              <TH>Fire Fighting Measure</TH>
              <TH>Present</TH>
              <TH>Factor</TH>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-300">
            <tr>
              <TD rowSpan={2} className="max-w-lg">
                Automatic Fire Suppression
              </TD>
              <TD className="max-w-lg">Automatic Water Extinguishing System</TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures
                  ?.automaticWaterExtinguishingSystem
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">
                {fireloadFactors.automaticWaterExtinguishingSystem}
              </TD>
            </tr>
            <tr>
              <TD className="max-w-lg">Independent Water Supplies</TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures?.independentWaterSupplies ||
                  0}
              </TD>
              <TD className="max-w-lg">
                {fireloadFactors.independentWaterSupplies}
              </TD>
            </tr>
            <tr>
              <TD rowSpan={3} className="max-w-lg">
                Automatic Fire Detection
              </TD>
              <TD className="max-w-lg">
                Automatic Fire Detection &amp; Alarm by Heat
              </TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures
                  ?.automaticDetectionAndAlarmHeat
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">
                {fireloadFactors.automaticDetectionAndAlarmHeat}
              </TD>
            </tr>
            <tr>
              <TD className="max-w-lg">
                Automatic Fire Detection &amp; Alarm by Smoke
              </TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures
                  ?.automaticDetectionAndAlarmSmoke
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">
                {fireloadFactors.automaticDetectionAndAlarmSmoke}
              </TD>
            </tr>
            <tr>
              <TD className="max-w-lg">
                Automatic Transmission to Fire Brigade
              </TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures
                  ?.automaticTransmissionToFireBrigade
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">
                {fireloadFactors.automaticTransmissionToFireBrigade}
              </TD>
            </tr>
            <tr>
              <TD rowSpan={5} className="max-w-lg">
                Manual Fire Supression
              </TD>
              <TD className="max-w-lg">Work Fire Brigade</TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures?.workFireBrigade
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">{fireloadFactors.workFireBrigade}</TD>
            </tr>
            <tr>
              <TD className="max-w-lg">Off Site Fire Brigade</TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures?.offsiteFireBrigade
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">{fireloadFactors.offsiteFireBrigade}</TD>
            </tr>
            <tr>
              <TD className="max-w-lg">Safe Access Routes</TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures?.safeAccessRoutes
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">{fireloadFactors.safeAccessRoutes}</TD>
            </tr>
            <tr>
              <TD className="max-w-lg">Fire Fighting Devices</TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures?.fireFightingDevices
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">
                {fireloadFactors.fireFightingDevices}
              </TD>
            </tr>
            <tr>
              <TD className="max-w-lg">Smoke and Exhaust System</TD>
              <TD className="max-w-lg">
                {fireloadInput.fireFightingMeasures?.smokeExhaustSystem
                  ? "Yes"
                  : "No"}
              </TD>
              <TD className="max-w-lg">{fireloadFactors.smokeExhaustSystem}</TD>
            </tr>
          </tbody>
        </Table>
      </CardBody>
    </Card>
  );
}

export function Item({ children }: { children: React.ReactNode }) {
  return (
    <li className="col-span-1">
      <div className="w-full flex flex-row items-center border rounded-lg p-4 hover:bg-gray-100 cursor-pointer active:bg-gray-200 shadow-md">
        {children}
      </div>
    </li>
  );
}

import { ParametricFireCalcSheet, ParametricFireInput } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import { InlineMath } from "react-katex";

export default function FireloadCalcCard({
  input,
  calculation,
}: {
  input: ParametricFireInput;
  calculation: ParametricFireCalcSheet;
}) {
  const factors = calculation.fireload;
  return (
    <Card>
      <div className="flex justify-center">
        <h2 className="text-2xl font-bold leading-7 sm:tracking-tight">
          Design Fireload Density
        </h2>
      </div>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Nomenclature
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <table>
          <thead>
            <tr>
              <th></th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>
                <InlineMath>{`q_{f,d}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  design fireload density (MJ/m<sup>2</sup>)
                </div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`q_{f,k}`}</InlineMath>
              </td>
              <td className="pl-4">
                <div>
                  characteristic fireload density (MJ/m<sup>2</sup>)
                </div>
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`A_{f}`}</InlineMath>
              </td>
              <td className="pl-4">
                compartment floor area (m<sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`m`}</InlineMath>
              </td>
              <td className="pl-4">combustion factor</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\delta_{q1}`}</InlineMath>
              </td>
              <td className="pl-4">
                fire activation risk factor due to the size of the compartment
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\delta_{q2}`}</InlineMath>
              </td>
              <td className="pl-4">
                fire activation risk factor due to the type of occupancy
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\delta_{n}`}</InlineMath>
              </td>
              <td className="pl-4">
                factor taking into account the different active fire fighting
                measures
              </td>
            </tr>
          </tbody>
        </table>
      </CardBody>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Equations
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <div className="flex flex-col gap-3">
          <div className="flex flex-row justify-left gap-3">
            <InlineMath>
              {`q_{f,d} = q_{f,k} \\cdot m \\cdot \\delta_{q1} \\cdot \\delta_{q2} \\cdot \\delta_{n}`}
            </InlineMath>
            (E.1)
          </div>
          <div className="flex flex-row gap-3 text-sm items-center">
            where:
            <InlineMath>
              {`\\delta_{n} = \\prod\\limits_{i=1}^{10} \\delta_{ni}`}
            </InlineMath>
          </div>
        </div>
      </CardBody>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Input
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <div className="flex flex-col sm:flex-row sm:gap-16">
          <table>
            <thead>
              <tr className="mt-4">
                <th></th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>
                  <InlineMath>{`q_{f,k}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {input.fireLoad.characteristicFireLoadDensity} MJ/m
                  <sup>2</sup>
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`m`}</InlineMath>
                </td>
                <td className="pl-4">{input.fireLoad.combustionFactor}</td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`A_{f}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {calculation.geometry.floorArea} m<sup>2</sup>
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{q1}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {calculation.fireload.deltaQ1} (Table E.1)
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{q2}`}</InlineMath>
                </td>
                <td className="pl-4">{input.fireLoad?.delta2}</td>
              </tr>
            </tbody>
          </table>
          <table>
            <thead>
              <tr className="mt-4">
                <th></th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n1}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {factors.deltaQ2.automaticWaterExtinguishingSystem}
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n2}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {factors.deltaQ2.independentWaterSupplies}
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n3}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {factors.deltaQ2.automaticDetectionAndAlarmHeat}
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n5}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {factors.deltaQ2.automaticDetectionAndAlarmSmoke}
                </td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n5}`}</InlineMath>
                </td>
                <td className="pl-4">
                  {factors.deltaQ2.automaticTransmissionToFireBrigade}
                </td>
              </tr>
            </tbody>
          </table>
          <table>
            <thead>
              <tr className="mt-4">
                <th></th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n6}`}</InlineMath>
                </td>
                <td className="pl-4">{factors.deltaQ2.workFireBrigade}</td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n7}`}</InlineMath>
                </td>
                <td className="pl-4">{factors.deltaQ2.offsiteFireBrigade}</td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n8}`}</InlineMath>
                </td>
                <td className="pl-4">{factors.deltaQ2.safeAccessRoutes}</td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n9}`}</InlineMath>
                </td>
                <td className="pl-4">{factors.deltaQ2.fireFightingDevices}</td>
              </tr>
              <tr>
                <td>
                  <InlineMath>{`\\delta_{n10}`}</InlineMath>
                </td>
                <td className="pl-4">{factors.deltaQ2.smokeExhaustSystem}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </CardBody>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Calculation
          </h3>
        </div>
      </CardHeader>
      <CardBody>
        <div className="flex flex-col gap-3">
          <div className="flex flex-row gap-3 text-sm">
            <InlineMath>
              {`\\delta_{n} = \\prod\\limits_{i=1}^{10} \\delta_{ni} = 
              ${factors.deltaQ2.automaticWaterExtinguishingSystem} \\cdot
              ${factors.deltaQ2.independentWaterSupplies} \\cdot
              ${factors.deltaQ2.automaticDetectionAndAlarmHeat} \\cdot
              ${factors.deltaQ2.automaticDetectionAndAlarmSmoke} \\cdot
              ${factors.deltaQ2.automaticTransmissionToFireBrigade} \\cdot
              ${factors.deltaQ2.workFireBrigade} \\cdot
              ${factors.deltaQ2.offsiteFireBrigade} \\cdot
              ${factors.deltaQ2.safeAccessRoutes} \\cdot
              ${factors.deltaQ2.fireFightingDevices} \\cdot
              ${factors.deltaQ2.smokeExhaustSystem}
              = ${factors.deltaQ2.product}`}
            </InlineMath>
          </div>
          <div className="flex flex-col items-left gap-3">
            <div className="flex flex-col items-left gap-3">
              <InlineMath>
                {`q_{f,d} = q_{f,k} \\cdot m \\cdot \\delta_{q1} \\cdot \\delta_{q2} \\cdot \\delta_{qn}`}
              </InlineMath>
              <InlineMath>
                {`q_{f,d} =
              ${input.fireLoad?.characteristicFireLoadDensity} \\cdot
              ${input.fireLoad?.combustionFactor} \\cdot
              ${calculation.fireload.deltaQ1} \\cdot
              ${input.fireLoad?.delta2} \\cdot
              ${factors.deltaQ2.product}`}
              </InlineMath>
              <div>
                <InlineMath>{`q_{f,d} = ${calculation.fireload.qFd}`}</InlineMath>{" "}
                MJ/m<sup>2</sup>
              </div>
            </div>
          </div>
        </div>
      </CardBody>
    </Card>
  );
}

import { Fragment } from "react";
import { Br187BasicOutput, SurfaceOutput, Tab } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import { InlineMath } from "react-katex";

export default function ViewFactors({ tab }: { tab: Tab }) {
  const data = tab.state as Br187BasicOutput;

  return (
    <Card>
      <div className="flex justify-center">
        <h2 className="text-2xl font-bold leading-7 sm:tracking-tight">
          Incident Heat Flux From Radiating Surfaces
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
                <InlineMath>{`I_{S}`}</InlineMath>
              </td>
              <td className="pl-4">
                intensity of radiation from a heated surface (kW/m<sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`I_{R}`}</InlineMath>
              </td>
              <td className="pl-4">
                intensity of radiation at the receiving surface (kW/m
                <sup>2</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\sigma`}</InlineMath>
              </td>
              <td className="pl-4">
                Stefan Boltzmann constant (5.67 x 10<sup>-11</sup> kW/m
                <sup>2</sup>/K<sup>4</sup>)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\epsilon`}</InlineMath>
              </td>
              <td className="pl-4">emissivity of the radiating object</td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`T`}</InlineMath>
              </td>
              <td className="pl-4">
                absolute temperature of the radiating object (K)
              </td>
            </tr>
            <tr>
              <td>
                <InlineMath>{`\\phi`}</InlineMath>
              </td>
              <td className="pl-4">view factor from heated surface</td>
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
          <InlineMath>{`I_{S} = \\sigma \\epsilon T^{4}`}</InlineMath>{" "}
          <InlineMath>{`I_{R} = \\phi I_{S}`}</InlineMath>{" "}
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
        <div className="grid grid-cols-2 sm:grid-cols-3 gap-4">
          {data.input.exposedSurfaces.map((surface) => {
            const surfaceCalc = data.calculation.surfaces.find(
              (s) => s.id === surface.id
            ) as SurfaceOutput;
            return (
              <Fragment key={surface.id}>
                <table>
                  <thead>
                    <tr className="mt-4">
                      <th></th>
                      <th></th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr className="flex flex-shrink">
                      <td>
                        <InlineMath>{`T_{${surface.id}}`}</InlineMath>
                      </td>
                      <td className="pl-4">{surface.temperature + 273.15} K</td>
                    </tr>
                    <tr className="flex flex-shrink">
                      <td>
                        <InlineMath>{`\\epsilon_{${surface.id}}`}</InlineMath>
                      </td>
                      <td className="pl-4">{surface.emissivity}</td>
                    </tr>
                    <tr className="flex flex-shrink">
                      <td>
                        <InlineMath>{`\\phi_{${surface.id}}`}</InlineMath>
                      </td>
                      <td className="pl-4">{surfaceCalc.phi}</td>
                    </tr>
                  </tbody>
                </table>
              </Fragment>
            );
          })}
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
        <div className="flex flex-col gap-7">
          {data.input.exposedSurfaces.map((surface) => {
            const surfaceCalc = data.calculation.surfaces.find(
              (s) => s.id === surface.id
            ) as SurfaceOutput;
            return (
              <div className="flex flex-col gap-3" key={surface.id}>
                <h1 className="text-lg font-semibold">{surfaceCalc.id}</h1>
                <div className="flex flex-row gap-3">
                  <InlineMath>{`I_{S} = \\sigma \\epsilon T^{4} = 5.67\\times10^{-1} \\cdot ${surface.emissivity} \\cdot ${surface.temperature + 273}^{4} = ${surfaceCalc.radiantHeatFlux} \\space kW/m^{2}`}</InlineMath>{" "}
                </div>
                <div className="flex flex-row gap-3">
                  <InlineMath>{`I_{R} = \\phi I_{S} = ${surfaceCalc.phi} \\cdot ${surfaceCalc.radiantHeatFlux} = ${surfaceCalc.incidentHeatFlux} \\space kW/m^{2}`}</InlineMath>{" "}
                </div>
              </div>
            );
          })}
          <h1 className="text-lg font-semibold">
            Radiation intensity at the receiving surface
          </h1>
          <div className="flex flex-col gap-3">
            <div className="flex flex-row gap-3">
              <InlineMath>{`I_{R\\space total} = \\sum{I_{R \\space surface}} = ${data.calculation.surfaces.map((s) => `${s.incidentHeatFlux > 0 ? s.incidentHeatFlux : `(${s.incidentHeatFlux})`}`).join("+")} = ${data.calculation.incidentHeatFlux} \\space kW/m^{2}`}</InlineMath>{" "}
            </div>
          </div>
        </div>
      </CardBody>
    </Card>
  );
}

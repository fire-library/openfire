import { useState, useEffect } from "react";
import { commands, Step, Method, Parameter, ParameterType } from "src/bindings";
import { stringedParam } from "src/bindingHelpers";
import { Card, CardBody, CardHeader } from "src/components";
import Calculation from "./Calculation";
import { InlineMath } from "react-katex";

function RenderStep({ step, index }: { step: Step; index: number }) {
  const [stepInputs, setStepInputs] = useState<ParameterType[]>([]);
  const [stepDeps, setStepDeps] = useState<ParameterType[]>([]);

  useEffect(() => {
    commands.getEquationInputsSymbols(index).then((inputs) => {
      if (inputs.status == "ok") {
        setStepInputs(inputs.data);
      }
    });
  }, [index]);

  useEffect(() => {
    commands.getEquationInputs(index).then((inputs) => {
      if (inputs.status == "ok") {
        setStepDeps(inputs.data);
      }
    });
  }, [index]);

  return (
    <Card key={step.name}>
      <div className="flex justify-center">
        <h2 className="text-2xl font-bold leading-7 sm:tracking-tight">
          {step.name}
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
            {stepInputs.map((parameter) => {
              const p = stringedParam(parameter);
              return (
                <tr key={p?.name}>
                  <td>
                    <InlineMath>{p?.id}</InlineMath>
                  </td>
                  <td className="pl-4">
                    {p?.name}{" "}
                    {p?.units && <InlineMath>{`(${p?.units})`}</InlineMath>}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </CardBody>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Input
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
            {stepDeps.map((parameter) => {
              const p = stringedParam(parameter);
              return (
                <tr key={p?.name}>
                  <td>
                    <InlineMath>{p?.id}</InlineMath>
                  </td>
                  <td className="pl-4">
                    <InlineMath>{`${p?.value} \\space ${p?.units ? p?.units : ""}`}</InlineMath>
                  </td>
                </tr>
              );
            })}
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
          {step.parameters.map((parameter) => {
            const p = stringedParam(parameter);
            return (
              <div className="flex flex-row gap-3" key={p?.id}>
                <Calculation parameter={parameter} type="symbols" />
              </div>
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
        <div className="flex flex-col gap-3">
          {step.parameters.map((parameter) => {
            const p = stringedParam(parameter);
            return (
              <div className="flex flex-row gap-3" key={p?.id}>
                <Calculation parameter={parameter} type="numbers" />
              </div>
            );
          })}
        </div>
      </CardBody>
    </Card>
  );
}

const shouldRenderStep = (step: Step) => {
  return (
    step.parameters.length > 0 &&
    step.parameters.every((p) => {
      const param = stringedParam(p);
      return param?.value !== null;
    })
  );
};

export default function ViewFactors({ method }: { method: Method }) {
  return (
    <>
      {method.calc_sheet.steps.map((step, index) =>
        shouldRenderStep(step) ? (
          <RenderStep step={step} index={index} key={step.name} />
        ) : null
      )}
    </>
  );
}

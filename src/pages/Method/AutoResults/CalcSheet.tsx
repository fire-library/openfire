import { useState, useEffect } from "react";
import { commands, Step, Method, Parameter } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import Calculation from "./Calculation";
import { InlineMath } from "react-katex";
import { getResultValue } from "../helpers";

function RenderStep({ step, index }: { step: Step; index: number }) {
  const [stepInputs, setStepInputs] = useState<Parameter[]>([]);
  const [stepDeps, setStepDeps] = useState<Parameter[]>([]);

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
              return (
                <tr key={parameter.name}>
                  <td>
                    <InlineMath>{parameter.id}</InlineMath>
                  </td>
                  <td className="pl-4">
                    {parameter.name}{" "}
                    {parameter.units && (
                      <InlineMath>{`(${parameter.units})`}</InlineMath>
                    )}
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
              return (
                <tr key={parameter.name}>
                  <td>
                    <InlineMath>{parameter.id}</InlineMath>
                  </td>
                  <td className="pl-4">
                    <InlineMath>{`${getResultValue(parameter.value)} ${parameter.units ? parameter.units : ""}`}</InlineMath>
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
            return (
              <div className="flex flex-row gap-3" key={parameter.id}>
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
            return (
              <div className="flex flex-row gap-3" key={parameter.id}>
                <Calculation parameter={parameter} type="numbers" />
              </div>
            );
          })}
        </div>
      </CardBody>
    </Card>
  );
}

export default function ViewFactors({ method }: { method: Method }) {
  return (
    <>
      {method.calc_sheet.steps.map((step, index) => (
        <RenderStep step={step} index={index} key={step.name} />
      ))}
    </>
  );
}

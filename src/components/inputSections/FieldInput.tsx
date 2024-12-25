import { FormStep, IntroComponent } from "src/bindings";
import { stringedParam } from "src/bindingHelpers";
import { Card, CardHeader, CardBody } from "src/components";
import Equation from "src/pages/Method/AutoResults/CalculationComponent/Equation";
import Input from "../tauriForm/Input";

export type InputProps = {
  step: FormStep;
  doQuickCalc?: () => void;
};

function IntroductionItem({ intro }: { intro: IntroComponent }) {
  if ("Text" in intro) {
    return (
      <div className="flex flex-row items-center">
        <p>{intro.Text}</p>
      </div>
    );
  } else if ("Title" in intro) {
    return (
      <h3 className="text-base font-semibold text-gray-900">{intro.Title}</h3>
    );
  } else if ("Equation" in intro) {
    if ("Equation" in intro.Equation) {
      return (
        <div className="my-2">
          <Equation equation={intro.Equation.Equation} />
        </div>
      );
    } else {
      return null;
    }
  } else {
    return null;
  }
}

export default function InputSection({ step, doQuickCalc }: InputProps) {
  const parameters = step.fields
    .map((field) => {
      if ("Individual" in field) {
        return field.Individual.parameter;
      } else {
        return null;
      }
    })
    .filter((field): field is Exclude<typeof field, null> => field !== null);

  return (
    <Card>
      <CardHeader>
        <div className="ml-4 mt-4 flex flex-col">
          <div className="flex flex-row items-center">
            <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
              {step.name}
            </h3>
          </div>
          <p className="mt-1 text-sm text-gray-500">{step.description}</p>
        </div>
      </CardHeader>
      <CardBody>
        {step.introduction.length > 0 && (
          <div className="border-white">
            <div className="flex flex-col">
              {step.introduction.map((intro, index) => (
                <div key={index} className="flex flex-row gap-10">
                  {intro.map((intro, index) => {
                    return <IntroductionItem key={index} intro={intro} />;
                  })}
                </div>
              ))}
            </div>
          </div>
        )}
        <div className="py-6 border-white">
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-9">
            {parameters.map((param) => {
              const p = stringedParam(param);
              if (p === null) {
                return null;
              } else
                return (
                  <Input key={p?.id} field={p} doQuickCalc={doQuickCalc} />
                );
            })}
          </div>
        </div>
      </CardBody>
    </Card>
  );
}

import { Parameter } from "src/bindings";
import { Card, CardHeader, CardBody } from "src/components";
import Input from "../tauriForm/Input";

export type InputProps = {
  title: string;
  description: string;
  parameters: Parameter[];
  doQuickCalc?: () => void;
};

export default function InputSection({
  parameters,
  title,
  description,
  doQuickCalc,
}: InputProps) {
  return (
    <Card>
      <CardHeader>
        <div className="ml-4 mt-4 flex flex-col">
          <div className="flex flex-row items-center">
            <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
              {title}
            </h3>
          </div>
          <p className="mt-1 text-sm text-gray-500">{description}</p>
        </div>
      </CardHeader>
      <CardBody>
        <div className="py-6 border-white">
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-9">
            {parameters.map((param) => (
              <Input key={param.id} field={param} doQuickCalc={doQuickCalc} />
            ))}
          </div>
        </div>
      </CardBody>
    </Card>
  );
}

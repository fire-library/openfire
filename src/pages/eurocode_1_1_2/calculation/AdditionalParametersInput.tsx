"use client";
import { UseFormRegister, FieldErrors } from "react-hook-form";
import { Input, Card, CardHeader, CardBody } from "src/components";
import { ParametricFireInput } from "src/bindings";

export default function FireLoadInput({
  register,
  errors,
}: {
  register: UseFormRegister<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
}) {
  return (
    <Card>
      <CardHeader>
        <div className="ml-4 mt-4">
          <h3 className="text-base font-semibold leading-6 text-gray-900">
            Additional parameters
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            Additional required parameters specific to the calculation.
          </p>
        </div>
      </CardHeader>
      <CardBody>
        <ul className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 py-4 w-full gap-y-8 gap-x-8">
          <li className="col-span-1 items-start flex flex-col">
            <Input
              name="timestep"
              register={register}
              errors={errors}
              type="number"
              required
              min={1}
              label="Timestep (s)"
            />
          </li>
        </ul>
      </CardBody>
    </Card>
  );
}

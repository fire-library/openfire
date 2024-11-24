"use client";
import {
  UseFormRegister,
  Control,
  UseFormWatch,
  FieldErrors,
  UseFormSetValue,
} from "react-hook-form";
import SelectMenu from "src/components/form/SelectMenu";
import { Input, Checkbox, Card, CardHeader, CardBody } from "src/components";
import { InlineMath } from "react-katex";
import { ParametricFireInput } from "src/bindings";

export default function FireLoadInput({
  register,
  control,
  watch,
  errors,
  setValue,
}: {
  register: UseFormRegister<ParametricFireInput>;
  control: Control<ParametricFireInput>;
  watch: UseFormWatch<ParametricFireInput>;
  errors: FieldErrors<ParametricFireInput>;
  setValue: UseFormSetValue<ParametricFireInput>;
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
        <div className="flex flex-col">
          <ul className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 py-4 w-full gap-y-8 gap-x-8">
            <li className="col-span-1 items-start flex flex-col">
              <Input
                name="fireLoad.characteristicFireLoadDensity"
                required
                min={1}
                register={register}
                errors={errors}
                type="number"
                label={
                  <>
                    Characteristic fire load density (MJ/m<sup>2</sup>):
                  </>
                }
              />
            </li>
            <li className="col-span-1 items-start flex flex-col">
              <SelectMenu
                label="Fire Growth Rate:"
                name="fireLoad.growthRate"
                control={control}
                fields={{ slow: "Slow", medium: "Medium", fast: "Fast" }}
              />
            </li>
            <li className="col-span-1 items-start flex flex-col">
              <Input
                name="fireLoad.combustionFactor"
                register={register}
                errors={errors}
                type="number"
                label="Combustion Factor"
                required
                min={0}
              />
            </li>
            <li className="col-span-1 items-start flex flex-col">
              <Input
                name="fireLoad.delta2"
                register={register}
                errors={errors}
                type="number"
                required
                min={0}
                label={
                  <>
                    <InlineMath>{String.raw`\delta_2`}</InlineMath> Risk of
                    activation due to occupancy type:
                  </>
                }
              />
            </li>
          </ul>
          <div className="w-full py-4">
            {/* <InlineMath>{String.raw`\delta_n`}</InlineMath> - Risk of activation
            based the active fire fighting measures available: */}
            <div className="flex flex-col my-4 gap-2">
              <div className="flex border px-4 py-2 items-center rounded-lg flex-col xl:flex-row">
                <h1 className="text-md font-medium w-40">
                  Automatic Fire Suppression
                </h1>
                <ul className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 py-4 w-full gap-2 items-center">
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.automaticWaterExtinguishingSystem"
                      setValue={setValue}
                      watch={watch}
                      label="Automatic Water Extinguishing System"
                    />
                  </li>
                  <li className="col-span-1">
                    <Input
                      name="fireLoad.fireFightingMeasures.independentWaterSupplies"
                      register={register}
                      errors={errors}
                      type="number"
                      label="Independent Water Supplies"
                      min={0}
                      required
                    />
                  </li>
                </ul>
              </div>
              <div className="flex border px-4 py-2 items-center rounded-lg flex-col xl:flex-row">
                <h1 className="text-md font-medium w-40">
                  Automatic Fire Detection
                </h1>
                <ul className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 py-4 w-full gap-2">
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.automaticDetectionAndAlarmHeat"
                      setValue={setValue}
                      label="Automatic fire Detection & Alarm by heat"
                      watch={watch}
                    />
                  </li>
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.automaticDetectionAndAlarmSmoke"
                      setValue={setValue}
                      label="Automatic fire Detection & Alarm by smoke"
                      watch={watch}
                    />
                  </li>
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.automaticTransmissionToFireBrigade"
                      setValue={setValue}
                      label="Automatic Alarm Transmission to Fire Brigade"
                      watch={watch}
                    />
                  </li>
                </ul>
              </div>
              <div className="flex border px-4 py-2 items-center rounded-lg flex-col xl:flex-row">
                <h1 className="text-md font-medium w-40">
                  Manual Fire Suppression
                </h1>
                <ul className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 py-4 w-full gap-2">
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.workFireBrigade"
                      setValue={setValue}
                      label="Work Fire Brigade"
                      watch={watch}
                    />
                  </li>
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.offsiteFireBrigade"
                      setValue={setValue}
                      label="Off Site Fire Brigade"
                      watch={watch}
                    />
                  </li>
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.safeAccessRoutes"
                      setValue={setValue}
                      label="Safe Access Routes"
                      watch={watch}
                    />
                  </li>
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.fireFightingDevices"
                      setValue={setValue}
                      label="Fire Fighting Devices"
                      watch={watch}
                    />
                  </li>
                  <li className="col-span-1">
                    <Checkbox
                      errors={errors}
                      control={control}
                      name="fireLoad.fireFightingMeasures.smokeExhaustSystem"
                      setValue={setValue}
                      label="Smoke Exhaust System"
                      watch={watch}
                    />
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </CardBody>
    </Card>
  );
}

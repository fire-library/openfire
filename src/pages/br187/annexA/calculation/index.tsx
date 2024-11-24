import { useEffect, useState } from "react";
import { Br187BasicInput, Br187BasicOutput, Tab, commands } from "src/bindings";
import { useForm } from "react-hook-form";
import { InformationCircleIcon } from "@heroicons/react/24/outline";
import SurfaceInput from "./ExposedSurfaceInput";
import QuickResults from "./QuickResults";
import { useTabs } from "src/tabs/tabProvider";

export default function Br187Basic({ tab }: { tab: Tab }) {
  const { tabs } = useTabs();
  const [quickOutput, setQuickOutput] = useState<Br187BasicOutput | null>(null);
  const tabIndex = tabs.findIndex((t) => t.id === tab?.id);
  const state = tab.state as Br187BasicInput;

  const {
    register,
    watch,
    control,
    formState: { errors },
    getValues,
    trigger,
    reset,
    setValue,
  } = useForm<Br187BasicInput>({
    mode: "onTouched",
    defaultValues: tab.state as Br187BasicInput,
  });

  const values = watch();

  useEffect(() => {
    state.id == values.id &&
      commands.updateTabFireAndForget(tab.id || "", {
        type: "Br187Basic",
        ...values,
      });
  }, [values, state.id, tab.id]);

  useEffect(() => {
    commands
      .performQuickCalc(values)
      .then((result) => {
        if (
          result.status == "ok" &&
          result.data.calculation.incidentHeatFlux !=
            quickOutput?.calculation.incidentHeatFlux
        ) {
          setQuickOutput(result.data);
        }
      })
      .catch(() => {
        setQuickOutput(null);
      });
  }, [values, quickOutput?.calculation.incidentHeatFlux]);

  useEffect(() => {
    reset(tab.state as Br187BasicInput);
  }, [tab.state, reset]);

  return (
    <>
      <form onSubmit={() => {}}>
        <div className="flex flex-row gap-4 justify-center max-w-6xl">
          <div className="flex flex-col flex-1">
            <div className="flex items-center justify-between pt-4 sm:pt-6 h-full">
              <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
                BR 187 Radiation View Factors{" "}
                <button
                  type="button"
                  onClick={() =>
                    commands.newTab(
                      {
                        type: "ParametricFireAbout",
                        id: "ParametricFireAbout",
                      },
                      tabIndex + 1
                    )
                  }
                >
                  <InformationCircleIcon className="h-6 w-6 text-gray-500" />
                </button>
              </h1>
            </div>
            <div className="flex items-center justify-between py-0 sm:pb-6">
              <h1 className="text-xl font-semibold leading-7 text-gray-500">
                Eurocode 1 Part 1-2 Annex A
              </h1>
            </div>
            <SurfaceInput
              setValue={setValue}
              trigger={trigger}
              register={register}
              control={control}
              watch={watch}
              errors={errors}
              getValues={getValues}
            />
            <QuickResults results={quickOutput} />
          </div>
        </div>
      </form>
    </>
  );
}

import { useEffect } from "react";
import { ParametricFireInput, Tab, commands } from "src/bindings";
import MaterialInput from "./MaterialInput";
import { useForm } from "react-hook-form";
import AdditionalParametersInput from "./AdditionalParametersInput";
import SurfaceInput from "./SurfaceInput";
import FireLoadInput from "./FireLoadInput";
import { InformationCircleIcon } from "@heroicons/react/24/outline";
import { useTabs } from "src/tabs/tabProvider";

export default function Calculation({ tab }: { tab: Tab }) {
  const { tabs } = useTabs();
  const tabIndex = tabs.findIndex((t) => t.id === tab?.id);
  const state = tab.state as ParametricFireInput;

  const {
    register,
    watch,
    control,
    formState: { errors },
    setValue,
    getValues,
    trigger,
    reset,
  } = useForm<ParametricFireInput>({
    mode: "onTouched",
    defaultValues: tab.state as ParametricFireInput,
  });

  const values = watch();

  useEffect(() => {
    state.id == values.id &&
      commands.updateTabFireAndForget(tab.id || "", {
        type: "ParametricFire",
        ...values,
      });
  }, [values, state.id, tab.id]);

  useEffect(() => {
    reset(tab.state as ParametricFireInput);
  }, [tab.state, reset]);

  return (
    <>
      <form onSubmit={() => {}}>
        <div className="flex flex-row gap-4 justify-center max-w-6xl">
          <div className="flex flex-col flex-1">
            <div className="flex items-center justify-between pt-4 sm:pt-6 h-full">
              <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
                Parametric Temperature-Time Curves{" "}
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
            <MaterialInput
              trigger={trigger}
              register={register}
              control={control}
              watch={watch}
              errors={errors}
              getValues={getValues}
            />
            <SurfaceInput
              control={control}
              watch={watch}
              errors={errors}
              setValue={setValue}
              trigger={trigger}
            />
            <FireLoadInput
              control={control}
              watch={watch}
              register={register}
              errors={errors}
              setValue={setValue}
            />
            <AdditionalParametersInput register={register} errors={errors} />
          </div>
        </div>
      </form>
    </>
  );
}

import { useState, ReactNode, useCallback, useEffect } from "react";
import { Tab, Method, commands } from "src/bindings";
import QuickResults from "./quickResults";
import FieldInputSection from "src/components/inputSections/FieldInput";
import Tabs from "../components/Tabs";
import CalcSheet from "./AutoResults/CalcSheet";
import ReactGA from "react-ga4";

export default function InputForm({ tab }: { tab: Tab }) {
  const [tabs, setTabs] = useState([
    {
      name: "Input",
      current: true,
      available: true,
    },
    {
      name: "Calculation",
      current: false,
      available: true,
    },
  ]);
  const state = tab.state as Method;

  const doQuickCalc = useCallback(() => {
    if (state.quick_calc_compatible) {
      commands.calculateForm();
    }
  }, [state.quick_calc_compatible]);

  useEffect(() => {
    ReactGA.send({
      hitType: "pageview",
      page: state.name,
    });
  }, [state.name]);

  useEffect(() => {
    if (tab.state.type === "Method") {
      if (tab.state.calc_sheet.stale) {
        setTabs([
          tabs[0],
          {
            ...tabs[1],
            available: false,
          },
        ]);
      } else {
        setTabs([
          tabs[0],
          {
            ...tabs[1],
            available: true,
          },
        ]);
      }
    }
  }, [tab.state]);

  return (
    <div className="flex flex-col max-w-6xl w-full">
      <div className="flex flex-row gap-4 justify-center max-w-6xl">
        <div className="flex flex-col flex-1">
          <div className="flex items-center justify-between pt-4 sm:pt-6 h-full">
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
              {state.name}
            </h1>
          </div>
          <div className="flex items-center justify-between py-0 sm:pb-6">
            <h1 className="text-xl font-semibold leading-7 text-gray-500">
              {state.reference.join(" | ")}
            </h1>
          </div>
        </div>
      </div>
      <Tabs tabs={tabs} setTabs={setTabs} />
      {tabs[0].current && (
        <>
          {state.form.steps.map((step) => {
            return (
              <FieldInputSection
                key={step.name}
                title={step.name}
                description={step.description}
                parameters={step.fields.map((field) => field.parameter)}
                doQuickCalc={doQuickCalc}
              />
            );
          })}
          <QuickResults results={state.calc_sheet} />
        </>
      )}

      {tabs[1].current && <CalcSheet method={state} />}
    </div>
  );
}

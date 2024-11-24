import "katex/dist/katex.min.css";
import { useEffect } from "react";
// import { TABS, TabNames } from "./constants";
import TabBar from "./Tabs";
import {
  register,
  unregister,
  isRegistered,
} from "@tauri-apps/plugin-global-shortcut";
import { useTabs } from "../../tabs/tabProvider";
import Calculation from "src/pages/eurocode_1_1_2/calculation";
import Br187Basic from "src/pages/br187/annexA/calculation";
import Br187BasicResults from "src/pages/br187/annexA/results";
import Results from "src/pages/eurocode_1_1_2/results";
import ParametricFireAbout from "src/pages/eurocode_1_1_2/about";
import PlumeHeatContent from "src/pages/Method";
import IndexPage from "src/pages";
import MethodBuilder from "src/pages/MethodBuilder";

export default function Explore() {
  const { currentTab, newTab } = useTabs();

  useEffect(() => {
    isRegistered("CommandOrControl+T").then((result) => {
      if (!result) {
        register("CommandOrControl+T", () => {
          newTab(null, null);
        });
      }
    });

    return () => {
      unregister("CommandOrControl+T");
    };
  }, [newTab]);

  return (
    <div className="flex flex-col h-full">
      <TabBar />
      <div className="mt-10 pb-10 flex items-center flex-col overflow-y-auto h-full">
        {currentTab === null && <div>Loading...</div>}
        {currentTab?.state.type == "Index" && <IndexPage />}
        {currentTab?.state.type == "ParametricFire" && (
          <Calculation tab={currentTab} />
        )}
        {currentTab?.state.type == "Br187Basic" && (
          <Br187Basic tab={currentTab} />
        )}
        {currentTab?.state.type == "ParametricFireResults" && (
          <Results tab={currentTab} />
        )}
        {currentTab?.state.type == "ParametricFireAbout" && (
          <ParametricFireAbout />
        )}
        {currentTab?.state.type == "Br187BasicResults" && (
          <Br187BasicResults tab={currentTab} />
        )}
        {currentTab?.state.type == "Method" && (
          <PlumeHeatContent tab={currentTab} />
        )}
        {currentTab?.state.type == "MethodBuilder" && (
          <MethodBuilder tab={currentTab} />
        )}
      </div>
    </div>
  );
}

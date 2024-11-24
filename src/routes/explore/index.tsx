import "katex/dist/katex.min.css";
import { useEffect } from "react";
import TabBar from "./Tabs";
import {
  register,
  unregister,
  isRegistered,
} from "@tauri-apps/plugin-global-shortcut";
import { useTabs } from "../../tabs/tabProvider";
import Method from "src/pages/Method";
import IndexPage from "src/pages";

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
        {currentTab?.state.type == "Method" && <Method tab={currentTab} />}
      </div>
    </div>
  );
}

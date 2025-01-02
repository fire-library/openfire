import "katex/dist/katex.min.css";
import { useEffect, useState } from "react";
import TabBar from "./Tabs";
import { useTabs } from "../../tabs/tabProvider";
import { useUpdate } from "src/tabs/useUpdate";
import Method from "src/pages/Method";
import IndexPage from "src/pages";
import { commands } from "src/bindings";
import ProgressBar from "src/components/ProgressBar";

export default function Explore() {
  const { currentTab } = useTabs();
  const update = useUpdate();
  const [version, setVersion] = useState<string | null>(null);

  useEffect(() => {
    commands.openfireVersion().then((v) => {
      if (v.status == "ok") {
        setVersion(v.data);
      }
    });
  }, []);

  useEffect(() => {
    update.checkForUpdate();
  }, []);

  return (
    <div className="flex flex-col h-full">
      <TabBar />
      <div className="mt-10 pb-10 flex items-center flex-col overflow-y-auto h-full">
        {currentTab === null && <div>Loading...</div>}
        {currentTab?.state.type == "Index" && <IndexPage />}
        {currentTab?.state.type == "Method" && <Method tab={currentTab} />}
      </div>
      <div className="flex flex-row h-5 border-y bg-slate-200 border-black justify-end w-full">
        {update.update?.available &&
        !update.installing &&
        !update.awaitingRestart ? (
          <button
            className="text-xs mr-2 hover:bg-gray-300 px-2"
            onClick={update.doUpdate}
          >
            Update to v{update.update.version}
          </button>
        ) : null}
        {update.installing ? (
          <div className="flex w-96">
            <ProgressBar percent={update.progress} />
          </div>
        ) : (
          ""
        )}
        <div className="mr-2 text-xs">{version ? `v${version}` : ""}</div>
      </div>
    </div>
  );
}

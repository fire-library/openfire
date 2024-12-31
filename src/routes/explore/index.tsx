import "katex/dist/katex.min.css";
import { useEffect, useState } from "react";
import TabBar from "./Tabs";
import { useTabs } from "../../tabs/tabProvider";
import Method from "src/pages/Method";
import IndexPage from "src/pages";
import { commands } from "src/bindings";

export default function Explore() {
  const { currentTab } = useTabs();
  const [version, setVersion] = useState<string | null>(null);

  useEffect(() => {
    commands.openfireVersion().then((v) => {
      if (v.status == "ok") {
        setVersion(v.data);
      }
    });
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
        <div className="mr-2 text-sm">{version ? `v${version}` : ""}</div>
      </div>
    </div>
  );
}

import "katex/dist/katex.min.css";
import TabBar from "./Tabs";
import { useTabs } from "../../tabs/tabProvider";
import Method from "src/pages/Method";
import IndexPage from "src/pages";

export default function Explore() {
  const { currentTab } = useTabs();

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

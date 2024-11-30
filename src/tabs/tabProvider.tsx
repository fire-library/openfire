import React, {
  ReactNode,
  createContext,
  useState,
  useContext,
  useEffect,
} from "react";
import { commands, Tab, TabState, Result } from "src/bindings";
import { listen } from "@tauri-apps/api/event";

const defaultTab: Tab = {
  id: "default",
  state: { type: "Index", id: "Index" },
  saved: true,
  current: true,
  filename: null,
  title: null,
};

export type TabContextType = {
  tabs: Tab[];
  currentTab: Tab;
  setCurrentTab: (_id: string) => Promise<Result<null, string>>;
  updateTab: (_id: string, _state: TabState) => Promise<Result<null, string>>;
  deleteTab: (_index: number) => Promise<Result<null, string>>;
  newTab: (
    _state: TabState | null,
    _after: number | null
  ) => Promise<Result<null, string>>;
};

const TabContext = createContext<TabContextType>({
  tabs: [],
  currentTab: defaultTab,
  setCurrentTab: commands.setCurrentTab,
  deleteTab: commands.deleteTab,
  newTab: commands.newTab,
  updateTab: commands.updateTab,
});

export const TabProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [tabs, setTabs] = useState<Tab[]>([]);
  const [currentTab, updateCurrentTab] = useState<Tab>(defaultTab);

  useEffect(() => {
    async function fetchTabs() {
      commands
        .getTabs()
        .then(
          (tab_state) => tab_state.status == "ok" && setTabs(tab_state.data)
        )
        .then(() => commands.getCurrentTab())
        .then((tab) => tab.status == "ok" && updateCurrentTab(tab.data));
    }

    fetchTabs();
  }, []);

  useEffect(() => {
    const unlisten = listen("tabs_updated", () => {
      commands
        .getTabs()
        .then(
          (tab_state) => tab_state.status == "ok" && setTabs(tab_state.data)
        )
        .then(() => commands.getCurrentTab())
        .then((tab) => tab.status == "ok" && updateCurrentTab(tab.data));
    });

    return () => {
      unlisten.then((f: () => void) => f());
    };
  }, []);

  const value = {
    tabs,
    currentTab: currentTab,
    setCurrentTab: commands.setCurrentTab,
    updateCurrentTab: updateCurrentTab,
    newTab: commands.newTab,
    deleteTab: commands.deleteTab,
    updateTab: commands.updateTab,
  };

  return <TabContext.Provider value={value}>{children}</TabContext.Provider>;
};

export function useTabs() {
  return useContext(TabContext);
}

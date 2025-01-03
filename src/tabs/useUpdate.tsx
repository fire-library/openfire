import { check, Update } from "@tauri-apps/plugin-updater";
import React, {
  ReactNode,
  createContext,
  useState,
  useContext,
  useCallback,
} from "react";

export type UpdateType = {
  checkForUpdate: () => void;
  progress: number;
  installing: boolean;
  awaitingRestart: boolean;
  update: Update | null;
  doUpdate: () => void;
};

const UpdateContext = createContext<UpdateType>({
  checkForUpdate: () => {},
  doUpdate: () => {},
  progress: 0,
  installing: false,
  awaitingRestart: false,
  update: null,
});

export const UpdateProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [update, setUpdate] = useState<Update | null>(null);
  const [awaitingRestart, setAwaitingRestart] = useState<boolean>(false);
  const [installing, setInstalling] = useState<boolean>(false);
  const [progress, setProgress] = useState<number>(0);

  const doUpdate = useCallback(async () => {
    if (!update) {
      return;
    }

    let size = 0;
    let stateProgress = 0;
    let localProgress = 0;
    let downloaded = 0;
    await update.download((event) => {
      switch (event.event) {
        case "Started":
          size = event.data.contentLength as number;
          setProgress(0);
          setInstalling(true);
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          localProgress = (downloaded / size) * 100;

          if (localProgress - stateProgress > 5) {
            stateProgress = localProgress;
            setProgress(localProgress);
          }
          break;
        case "Finished":
          setInstalling(false);
          setAwaitingRestart(true);
          break;
      }

      setUpdate(null);
    });
  }, [update]);

  const checkForUpdate = useCallback(async () => {
    const updateFound = await check();
    if (updateFound) {
      setUpdate(updateFound);
    }
  }, []);

  const value = {
    checkForUpdate,
    progress,
    installing,
    awaitingRestart,
    update,
    doUpdate,
  };

  return (
    <UpdateContext.Provider value={value}>{children}</UpdateContext.Provider>
  );
};

export function useUpdate() {
  return useContext(UpdateContext);
}

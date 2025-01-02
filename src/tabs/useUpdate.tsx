import { check } from "@tauri-apps/plugin-updater";
import React, {
  ReactNode,
  createContext,
  useState,
  useContext,
  useEffect,
  useCallback,
} from "react";
import { commands } from "src/bindings";

export type UpdateType = {
  checkForUpdate: () => void;
};

const UpdateContext = createContext<UpdateType>({
  checkForUpdate: () => {},
});

export const UpdateProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const checkForUpdate = async () => {};

  const value = {
    checkForUpdate,
  };

  return (
    <UpdateContext.Provider value={value}>{children}</UpdateContext.Provider>
  );
};

export function useUserAgreement() {
  return useContext(UpdateContext);
}

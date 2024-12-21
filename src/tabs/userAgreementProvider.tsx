import React, {
  ReactNode,
  createContext,
  useState,
  useContext,
  useEffect,
} from "react";
import { Tab } from "src/bindings";

const defaultTab: Tab = {
  id: "default",
  state: { type: "Index", id: "Index" },
  saved: true,
  current: true,
  filename: null,
  title: null,
};

export type UserAgreementType = {
  agreed?: boolean;
  agree: () => void;
};

const UserAgreementContext = createContext<UserAgreementType>({
  agreed: undefined,
  agree: () => {},
});

export const UserAgreementProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [agreed, setAgreed] = useState<boolean | undefined>(false);

  useEffect(() => {}, []);

  const value = {
    agreed,
    agree: () => setAgreed(true),
  };

  return (
    <UserAgreementContext.Provider value={value}>
      {children}
    </UserAgreementContext.Provider>
  );
};

export function useUserAgreement() {
  return useContext(UserAgreementContext);
}

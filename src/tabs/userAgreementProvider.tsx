import React, {
  ReactNode,
  createContext,
  useState,
  useContext,
  useEffect,
  useCallback,
} from "react";
import { Tab, commands } from "src/bindings";

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

  const agree = useCallback(async () => {
    commands.agreeToLicense().then((agreed) => {
      console.log(agreed);
      if (agreed.status == "ok") {
        setAgreed(true);
      }
    });
  }, []);

  useEffect(() => {
    commands.hasAgreedToLatestLicense().then((agreed) => {
      console.log("effect: ", agreed);
      if (agreed.status == "ok") {
        setAgreed(agreed.data);
      }
    });
  }, []);

  const value = {
    agreed,
    agree,
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
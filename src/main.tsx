import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import Root from "./routes/Root";
import "./styles.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { TabProvider } from "src/tabs/tabProvider";
import { UserAgreementProvider } from "src/tabs/userAgreementProvider";
import { UpdateProvider } from "src/tabs/useUpdate";
import ReactGA from "react-ga4";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

ReactGA.initialize(import.meta.env.VITE_GA_TRACKING_ID);

const router = createBrowserRouter([
  {
    path: "/",
    id: "root",
    element: <Root />,
  },
]);

function App() {
  return (
    <UserAgreementProvider>
      <UpdateProvider>
        <TabProvider>
          <RouterProvider router={router} />
        </TabProvider>
      </UpdateProvider>
    </UserAgreementProvider>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

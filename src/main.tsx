import React from "react";
import ReactDOM from "react-dom/client";
import Root from "./routes/Root";
import "./styles.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { TabProvider } from "src/tabs/tabProvider";
import ReactGA from "react-ga4";

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
    <TabProvider>
      <RouterProvider router={router} />
    </TabProvider>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

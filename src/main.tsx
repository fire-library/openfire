import React, { useEffect } from "react";
import ReactDOM from "react-dom/client";
import Root from "./routes/Root";
import "./styles.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { TabProvider } from "src/tabs/tabProvider";
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
  useEffect(() => {
    const checkForUpdate = async () => {
      if (import.meta.env.ENVIRONMENT === "release") {
        const update = await check();
        if (update) {
          console.log(
            `found update ${update.version} from ${update.date} with notes ${update.body}`
          );
          let downloaded = 0;
          let contentLength: number | undefined = 0;
          await update.downloadAndInstall((event) => {
            switch (event.event) {
              case "Started":
                contentLength = event.data.contentLength;
                console.log(
                  `started downloading ${event.data.contentLength} bytes`
                );
                break;
              case "Progress":
                downloaded += event.data.chunkLength;
                console.log(`downloaded ${downloaded} from ${contentLength}`);
                break;
              case "Finished":
                console.log("download finished");
                alert("Download finished, please restart the app");
                break;
            }
          });

          await relaunch();
        }
      }
    };

    checkForUpdate();
  }, []);

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

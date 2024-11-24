import { useState, ReactNode, Fragment } from "react";
import { Br187BasicOutput, Tab } from "src/bindings";
import { Card, CardBody, CardHeader } from "src/components";
import Table, { TH, TD } from "src/components/form/Table";
import Tabs from "../../../components/Tabs";
import { viewFactorText } from "../utils";
import ViewFactors from "./viewFactors";
import HeatFlux from "./HeatFlux";

export default function Results({ tab }: { tab: Tab }) {
  const tabs: ReactNode[] = ["Calculation", "Input"];
  const data = tab.state as Br187BasicOutput;
  const [currentTab, setCurrentTab] = useState(tabs[0]);

  return (
    <div className="flex flex-row max-w-6xl w-full px-0 sm:px-5">
      <div className="flex flex-col w-full">
        <div className="flex items-center justify-between pt-4 sm:pt-6">
          <h1 className="text-2xl font-semibold leading-7 text-gray-900">
            BR 187 Radiation View Factors - Results
          </h1>
        </div>
        <div className="flex items-center justify-between py-0 sm:pb-6">
          <h1 className="text-xl font-semibold leading-7 text-gray-500">
            BR 187 Annex A
          </h1>
        </div>
        <Tabs tabs={tabs} currentTab={currentTab} setTab={setCurrentTab} />
        {currentTab === tabs[0] && (
          <>
            <ViewFactors tab={tab} />
            <HeatFlux tab={tab} />
          </>
        )}
        {currentTab === tabs[1] && (
          <div className="flex flex-col">
            <Card>
              <CardHeader>
                <div className="ml-4 mt-4 flex flex-col">
                  <div className="flex flex-row items-center">
                    <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                      View Factors
                    </h3>
                  </div>
                  <p className="mt-1 text-sm text-gray-500">
                    A list of view factors that the receiver is exposed to.
                  </p>
                </div>
              </CardHeader>
              <CardBody>
                <Table>
                  <thead>
                    <tr>
                      <TH>ID</TH>
                      <TH>View Factor Type</TH>
                      <TH>Height (m)</TH>
                      <TH>Width (m)</TH>
                      <TH>Distance to receiver (m)</TH>
                      <TH>Temperature (C)</TH>
                      <TH>Emissivity</TH>
                      <TH>Positive?</TH>
                      <TH></TH>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-300">
                    {data.input.exposedSurfaces.map((surface, index) => (
                      <tr key={index}>
                        <TD>{surface.id}</TD>
                        <TD>{viewFactorText(surface.viewFactorType)}</TD>
                        <TD>{surface.height}</TD>
                        <TD>{surface.width}</TD>
                        <TD>{surface.distance}</TD>
                        <TD>{surface.temperature}</TD>
                        <TD>{surface.emissivity}</TD>
                        <TD>{surface.additive ? "Yes" : "No"}</TD>
                      </tr>
                    ))}
                  </tbody>
                </Table>
              </CardBody>
            </Card>
          </div>
        )}
      </div>
    </div>
  );
}

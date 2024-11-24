import { useEffect } from "react";
import { Tab, MethodBuilder, commands } from "src/bindings";
import { Card, CardHeader, CardBody } from "src/components";
import { TrashIcon, PlusIcon } from "@heroicons/react/20/solid";
import Input from "./Input";
import TextArea from "./TextArea";
import Checkbox from "./Checkbox";
import { loader } from "@monaco-editor/react";

export default function InputForm({ tab }: { tab: Tab }) {
  const state = tab.state as MethodBuilder;

  useEffect(() => {
    loader.init().then((monaco) => {
      const wrapper = document.getElementById("code_editor");
      if (!wrapper) return;
      if (wrapper.childNodes.length > 0) return;

      monaco.languages.register({ id: "simpleLang" });

      monaco.languages.setMonarchTokensProvider("simpleLang", {
        // Set defaultToken to invalid to mark unmatched content
        defaultToken: "invalid",

        // Define token types
        keywords: ["fn", "var", "return", "if", "else"],

        // Define the tokenizer
        tokenizer: {
          root: [
            // Recognize keywords
            [/\b(fn|return|if|else|cond|do)\b/, "keyword"],

            // Recognize numbers
            [/\b\d+(\.\d+)?\b/, "number"],

            // Recognize comments
            [/\/\/.*/, "comment"],

            // Recognize strings
            [/"[^"]*"/, "string"],

            [/[a-zA-Z_$][\w$]*/, "identifier"],
          ],
        },
      });

      // Define the language configuration (e.g., comments)
      monaco.languages.setLanguageConfiguration("simpleLang", {
        comments: {
          lineComment: "//",
        },
        brackets: [
          ["{", "}"],
          ["[", "]"],
          ["(", ")"],
        ],
      });

      wrapper.style.height = "25vh";
      const properties = {
        value:
          'function hello() {\n\talert("Hellsasdfjkhaslkfjhsado world!");\n}',
        language: "simpleLang",
      };

      monaco.editor.create(wrapper, properties);
    });
  }, []);

  return (
    <div className="flex flex-col max-w-6xl w-full">
      <div className="flex flex-row gap-4 justify-center max-w-6xl">
        <div className="flex flex-col flex-1">
          <div className="flex items-center justify-between pt-4 sm:pt-6 h-full">
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
              Method Builder
            </h1>
          </div>
        </div>
      </div>

      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                Method Details
              </h3>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              The high level details of the method
            </p>
          </div>
        </CardHeader>
        <CardBody>
          <div className="py-6 border-white">
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-9">
              <Input
                id="name"
                label="Method Name"
                defaultValue={state.name}
                className="w-full"
              />
              <Input
                id="reference"
                label="Reference"
                defaultValue={state.reference.join(", ")}
                className="w-full"
              />
              <TextArea label="Description" className="w-full col-span-2" />
            </div>
          </div>
        </CardBody>
      </Card>

      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                Metadata
              </h3>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              Additional Metadata that should be provided when running the
              method
            </p>
          </div>
        </CardHeader>
        <CardBody>
          <div className="py-6 border-white">
            <button
              className="bg-indigo-500 text-white hover:bg-indigo-700 p-2 rounded"
              onClick={() => commands.methodBuilderAddMetadata()}
            >
              <PlusIcon className="h-5 w-5" />
            </button>
            <table className="w-full">
              <thead>
                <tr>
                  <th className="text-left">Name</th>
                  <th className="text-left">Required</th>
                  <th className="text-left"></th>
                </tr>
              </thead>
              <tbody>
                {state.metadata.map((metadata) => {
                  return (
                    <tr key={metadata.id}>
                      <td>
                        <Input
                          id={metadata.id}
                          updateFunction={(id: string, value: string) =>
                            commands.methodBuilderUpdateMetadataName(id, value)
                          }
                          defaultValue={metadata.name}
                          className="w-full pr-5"
                        />
                      </td>
                      <td>
                        <Checkbox className="w-full" />
                      </td>
                      <td>
                        <button
                          className="bg-red-500 text-white hover:bg-red-700 p-2 rounded"
                          onClick={() =>
                            commands.methodBuilderDeleteMetadata(metadata.id)
                          }
                        >
                          <TrashIcon className="h-5 w-5" />
                        </button>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </CardBody>
      </Card>

      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                Equations
              </h3>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              All of the equations used in the method.
            </p>
          </div>
        </CardHeader>
        <CardBody>
          <div className="py-6 border-white">
            <div id="code_editor" className="w-50" />
          </div>
        </CardBody>
      </Card>

      <Card>
        <CardHeader>
          <div className="ml-4 mt-4 flex flex-col">
            <div className="flex flex-row items-center">
              <h3 className="text-base font-semibold leading-6 text-gray-900 mr-2">
                Parameters
              </h3>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              All of the parameters used in the method.
            </p>
          </div>
        </CardHeader>
        <CardBody>
          <div className="py-6 border-white"></div>
        </CardBody>
      </Card>
    </div>
  );
}

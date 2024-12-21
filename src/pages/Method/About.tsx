import { useState, ReactNode, useCallback, useEffect } from "react";
import { Document, commands } from "src/bindings";
import Markdown from "react-markdown";

export default function About({
  document,
  methodName,
}: {
  document: Document;
  methodName: string;
}) {
  const markdown = "jakshdfjkshd";
  const [docName, setDocName] = useState<string>("");
  const [documentDescription, setDocumentDescription] = useState<string>("");
  const [methodDescription, setMethodDescription] = useState<string>("");
  const [methodLimitations, setMethodLimitations] = useState<string>("");

  useEffect(() => {
    const getName = async () =>
      commands.documentTitle(document).then((name) => {
        if (name.status == "ok") {
          setDocName(name.data);
        }
        console.log(name);
      });

    getName();
  }, []);

  useEffect(() => {
    const getDescription = async () =>
      commands.aboutDocument(document).then((name) => {
        if (name.status == "ok") {
          setDocumentDescription(name.data);
        }
        console.log(name);
      });

    getDescription();
  }, []);

  useEffect(() => {
    const getMethodDescription = async () =>
      commands.aboutMethod(document).then((name) => {
        if (name.status == "ok") {
          setMethodDescription(name.data);
        }
        console.log(name);
      });

    getMethodDescription();
  }, []);

  useEffect(() => {
    const getLimitaitons = async () =>
      commands.methodLimitations(document).then((name) => {
        if (name.status == "ok") {
          setMethodLimitations(name.data);
        }
        console.log(name);
      });

    getLimitaitons();
  }, []);
  return (
    <div className="flex flex-col gap-5 mt-10">
      <div className="flex flex-row items-center gap-10">
        <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
          {docName}
        </h1>
      </div>
      <Markdown
        components={{
          h1: ({ children }) => (
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
              {children}
            </h1>
          ),
        }}
      >
        {documentDescription}
      </Markdown>
      <div className="flex flex-row items-center gap-10">
        <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
          Method Description
        </h1>
      </div>
      <Markdown
        components={{
          h1: ({ children }) => (
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
              {children}
            </h1>
          ),
        }}
      >
        {methodDescription}
      </Markdown>
      <div className="flex flex-row items-center gap-10">
        <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
          Method Limitations
        </h1>
      </div>
      <Markdown
        components={{
          h1: ({ children }) => (
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
              {children}
            </h1>
          ),
        }}
      >
        {methodDescription}
      </Markdown>
    </div>
  );
}

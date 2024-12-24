import { useState, ReactNode, useCallback, useEffect } from "react";
import { Document, commands } from "src/bindings";
import Markdown from "src/components/Markdown";

export default function About({
  document,
  methodName,
}: {
  document: Document;
  methodName: string;
}) {
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
      });

    getName();
  }, []);

  useEffect(() => {
    const getDescription = async () =>
      commands.aboutDocument(document).then((name) => {
        if (name.status == "ok") {
          setDocumentDescription(name.data);
        }
      });

    getDescription();
  }, []);

  useEffect(() => {
    const getMethodDescription = async () =>
      commands.aboutMethod(document).then((name) => {
        if (name.status == "ok") {
          setMethodDescription(name.data);
        }
      });

    getMethodDescription();
  }, []);

  useEffect(() => {
    const getLimitations = async () =>
      commands.methodLimitations(document).then((name) => {
        if (name.status == "ok") {
          setMethodLimitations(name.data);
        }
      });

    getLimitations();
  }, []);
  return (
    <div className="flex flex-col gap-5 mt-10">
      <div className="flex flex-row items-center gap-10">
        <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
          {docName}
        </h1>
      </div>
      <Markdown>{documentDescription}</Markdown>
      <div className="flex flex-row items-center gap-10">
        <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
          Method Description
        </h1>
      </div>
      <Markdown>{methodDescription}</Markdown>
      <div className="flex flex-row items-center gap-10">
        <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
          Method Limitations
        </h1>
      </div>
      <Markdown>{methodLimitations}</Markdown>
    </div>
  );
}

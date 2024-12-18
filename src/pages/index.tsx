import { useState, useEffect } from "react";
import { FireIcon } from "@heroicons/react/24/outline";
import {
  commands,
  Implementation,
  DocumentImplementations,
} from "src/bindings";
import Fuse from "fuse.js";
import { Card, CardHeader, CardBody } from "src/components";

export function ListItem({
  implementation,
}: {
  implementation: Implementation;
}) {
  const [friendly_reference, setFriendlyReference] = useState("");

  useEffect(() => {
    const ref = async () =>
      commands.friendlyReference(implementation.reference).then((reference) => {
        if (reference.status == "ok") {
          setFriendlyReference(reference.data);
        }
      });

    ref();
  }, [implementation.reference]);
  const Icon = FireIcon;

  return (
    <li className="col-span-1 divide-y divide-gray-200 rounded-lg shadow-md bg-white w-full hover:bg-gray-100 border">
      <button
        className="w-full h-full"
        onClick={() => {
          if (implementation.method_type) {
            commands.setCurrentTabMethod(implementation.method_type);
          }
        }}
      >
        <div className="flex w-full space-x-6 px-6 py-4">
          <div className="flex-1">
            <div className="flex flex-col items-start mb-2">
              <h3 className="text-sm font-medium text-indigo-600 text-start">
                {implementation.name}
              </h3>
              <p className="mt-1 text-sm text-gray-500 text-start">
                {friendly_reference}
              </p>
            </div>
            <div className="flex gap-1 overflow-hidden">
              {implementation.tags.map((tag) => (
                <span
                  key={tag}
                  className="inline-flex flex-shrink-0 items-center rounded-full bg-green-50 px-1.5 py-0.5 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20"
                >
                  {tag}
                </span>
              ))}
            </div>
          </div>
        </div>
      </button>
    </li>
  );
}

function IndexPage() {
  const [search, setSearch] = useState("");
  const [allMethods, setAllMethods] = useState<DocumentImplementations[]>([]);
  const [searchedMethods, setSearchedMethods] = useState<
    Implementation[] | null
  >([]);
  const [fuse] = useState(
    new Fuse(allMethods.map((doc) => doc.implementations).flat(), {
      keys: ["name", "search_reference", "tags"],
    })
  );

  useEffect(() => {
    const all_impls = async () =>
      commands.allImplementations().then((implementations) => {
        if (implementations.status == "ok") {
          const ordered = implementations.data.sort((a, b) => {
            return a.document.localeCompare(b.document);
          });
          setAllMethods(ordered);
        }
      });

    all_impls();
  }, []);

  useEffect(() => {
    fuse.setCollection(allMethods.map((doc) => doc.implementations).flat());
  }, [allMethods]);

  useEffect(() => {
    if (search.trim() === "") {
      setSearchedMethods(null);
    } else {
      setSearchedMethods(fuse.search(search).map((result) => result.item));
    }
  }, [allMethods, search, fuse]);

  return (
    <div className="max-w-5xl w-full">
      <form className="max-w-lg mx-auto mt-10">
        <div className="flex">
          <div className="relative w-full">
            <input
              type="search"
              id="search-dropdown"
              className="block p-2.5 w-full z-20 text-sm text-gray-900 bg-gray-50 rounded-lg border-1 border border-gray-300"
              placeholder="Search"
              onChange={(e) => setSearch(e.target.value)}
              required
            />
          </div>
        </div>
      </form>

      {searchedMethods ? (
        <>
          <div className="flex flex-col flex-1 w-full mt-10">
            <div className="flex pt-4 sm:pt-6 h-full">
              <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
                Search Results
              </h1>
            </div>
          </div>
          <ul className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 mt-10">
            {searchedMethods.map((implementation, index) => {
              const Icon = FireIcon;
              return <ListItem key={index} implementation={implementation} />;
            })}
          </ul>
        </>
      ) : (
        <div className="flex flex-col">
          {allMethods.map((doc, index) => {
            return (
              <Card key={index}>
                <CardHeader>
                  <div className="flex flex-col">
                    <h3 className="text-base font-semibold text-gray-900">
                      {doc.document}
                    </h3>
                    <p className="mt-1 text-sm text-gray-500">
                      Harvard Reference
                    </p>
                  </div>
                </CardHeader>
                <CardBody>
                  <ul className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 w-full">
                    {doc.implementations.map((implementation, index) => {
                      return (
                        <ListItem key={index} implementation={implementation} />
                      );
                    })}
                  </ul>
                </CardBody>
              </Card>
            );
          })}
        </div>
      )}
    </div>
  );
}

export default IndexPage;

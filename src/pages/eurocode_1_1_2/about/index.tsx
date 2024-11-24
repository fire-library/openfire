"use client";

import "katex/dist/katex.min.css";
import Markdown from "./about.mdx";
import { components } from "src/mdx/components";

export default function Eurocode_1_1_2() {
  return (
    <div className="max-w-6xl">
      <Markdown components={components} />
    </div>
  );
}

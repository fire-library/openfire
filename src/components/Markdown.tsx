import Markdown from "react-markdown";
import rehypeKatex from "rehype-katex";
import remarkMath from "remark-math";

export default function MyMarkdown({ children }: { children: string | null }) {
  return (
    <Markdown
      rehypePlugins={[rehypeKatex]}
      remarkPlugins={[remarkMath]}
      components={{
        h1: ({ children }) => (
          <h1 className="text-2xl font-semibold leading-7 text-gray-900 flex flex-row">
            {children}
          </h1>
        ),
      }}
    >
      {children}
    </Markdown>
  );
}

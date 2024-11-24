import type { MDXComponents } from "mdx/types";
import { DetailedHTMLProps, HTMLAttributes } from "react";

function H1({
  children,
}: DetailedHTMLProps<HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement>) {
  return <h1 className="text-4xl font-bold py-6">{children}</h1>;
}

function H2({
  children,
}: DetailedHTMLProps<HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement>) {
  return <h1 className="text-2xl font-bold pb-3">{children}</h1>;
}

function Paragraph({
  children,
}: DetailedHTMLProps<
  HTMLAttributes<HTMLParagraphElement>,
  HTMLParagraphElement
>) {
  return <p className="text-left text-black pb-3">{children}</p>;
}

export const components: MDXComponents = {
  h1: H1,
  h2: H2,
  p: Paragraph,
};

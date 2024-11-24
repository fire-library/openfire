/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  query InitSession($token: String!) {\n    initSession(token: $token) {\n      user {\n        firstName\n        lastName\n        email\n      }\n    }\n  }\n": types.InitSessionDocument,
    "\n  query CheckSession {\n    user {\n      email\n      firstName\n      lastName\n    }\n  }\n": types.CheckSessionDocument,
    "\n  query Logout {\n    logout {\n      success\n    }\n  }\n": types.LogoutDocument,
    "\n  mutation Ask($convId: ID, $question: String!, $page: String!, $tabId: ID!) {\n    ask(\n      conversationId: $convId\n      question: $question\n      page: $page\n      tabId: $tabId\n    ) {\n      response {\n        id\n        text\n        isUser\n      }\n    }\n  }\n": types.AskDocument,
    "\n  query ConversationByTabId($tabId: ID!) {\n    user {\n      conversationByTabId(tabId: $tabId) {\n        id\n        messages {\n          id\n          text\n          isUser\n        }\n      }\n    }\n  }\n": types.ConversationByTabIdDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query InitSession($token: String!) {\n    initSession(token: $token) {\n      user {\n        firstName\n        lastName\n        email\n      }\n    }\n  }\n"): (typeof documents)["\n  query InitSession($token: String!) {\n    initSession(token: $token) {\n      user {\n        firstName\n        lastName\n        email\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query CheckSession {\n    user {\n      email\n      firstName\n      lastName\n    }\n  }\n"): (typeof documents)["\n  query CheckSession {\n    user {\n      email\n      firstName\n      lastName\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Logout {\n    logout {\n      success\n    }\n  }\n"): (typeof documents)["\n  query Logout {\n    logout {\n      success\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation Ask($convId: ID, $question: String!, $page: String!, $tabId: ID!) {\n    ask(\n      conversationId: $convId\n      question: $question\n      page: $page\n      tabId: $tabId\n    ) {\n      response {\n        id\n        text\n        isUser\n      }\n    }\n  }\n"): (typeof documents)["\n  mutation Ask($convId: ID, $question: String!, $page: String!, $tabId: ID!) {\n    ask(\n      conversationId: $convId\n      question: $question\n      page: $page\n      tabId: $tabId\n    ) {\n      response {\n        id\n        text\n        isUser\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query ConversationByTabId($tabId: ID!) {\n    user {\n      conversationByTabId(tabId: $tabId) {\n        id\n        messages {\n          id\n          text\n          isUser\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query ConversationByTabId($tabId: ID!) {\n    user {\n      conversationByTabId(tabId: $tabId) {\n        id\n        messages {\n          id\n          text\n          isUser\n        }\n      }\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;
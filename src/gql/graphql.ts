/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * The `DateTime` scalar type represents a DateTime
   * value as specified by
   * [iso8601](https://en.wikipedia.org/wiki/ISO_8601).
   */
  DateTime: { input: any; output: any; }
  /**
   * Leverages the internal Python implementation of UUID (uuid.UUID) to provide native UUID objects
   * in fields, resolvers and input.
   */
  UUID: { input: any; output: any; }
};

export type AskQuestion = {
  __typename?: 'AskQuestion';
  ok?: Maybe<Scalars['Boolean']['output']>;
  response?: Maybe<MessageType>;
};

/** An enumeration. */
export enum ConversationSort {
  EarliestFirst = 'EARLIEST_FIRST',
  LatestFirst = 'LATEST_FIRST'
}

export type ConversationType = {
  __typename?: 'ConversationType';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['UUID']['output'];
  messages: Array<MessageType>;
  tabId?: Maybe<Scalars['UUID']['output']>;
  title: Scalars['String']['output'];
  user: UserType;
};


export type ConversationTypeMessagesArgs = {
  sort?: InputMaybe<MessageSort>;
};

export type LogoutType = {
  __typename?: 'LogoutType';
  success?: Maybe<Scalars['Boolean']['output']>;
};

/** An enumeration. */
export enum MessageSort {
  EarliestFirst = 'EARLIEST_FIRST',
  LatestFirst = 'LATEST_FIRST'
}

export type MessageType = {
  __typename?: 'MessageType';
  context?: Maybe<Scalars['String']['output']>;
  conversation: ConversationType;
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['UUID']['output'];
  isUser: Scalars['Boolean']['output'];
  text: Scalars['String']['output'];
  user: UserType;
};

export type Mutations = {
  __typename?: 'Mutations';
  ask?: Maybe<AskQuestion>;
};


export type MutationsAskArgs = {
  conversationId?: InputMaybe<Scalars['ID']['input']>;
  page?: Scalars['String']['input'];
  question: Scalars['String']['input'];
  tabId: Scalars['ID']['input'];
};

export type OneTimeTokenType = {
  __typename?: 'OneTimeTokenType';
  user: UserType;
};

export type Query = {
  __typename?: 'Query';
  devLogin?: Maybe<UserType>;
  initSession?: Maybe<OneTimeTokenType>;
  logout?: Maybe<LogoutType>;
  user?: Maybe<UserType>;
};


export type QueryDevLoginArgs = {
  id: Scalars['ID']['input'];
};


export type QueryInitSessionArgs = {
  token: Scalars['String']['input'];
};

export type UserType = {
  __typename?: 'UserType';
  conversationById?: Maybe<ConversationType>;
  conversationByTabId?: Maybe<ConversationType>;
  conversations?: Maybe<Array<Maybe<ConversationType>>>;
  email: Scalars['String']['output'];
  firstName: Scalars['String']['output'];
  lastName: Scalars['String']['output'];
};


export type UserTypeConversationByIdArgs = {
  id: Scalars['ID']['input'];
};


export type UserTypeConversationByTabIdArgs = {
  tabId: Scalars['ID']['input'];
};


export type UserTypeConversationsArgs = {
  sort?: ConversationSort;
};

export type InitSessionQueryVariables = Exact<{
  token: Scalars['String']['input'];
}>;


export type InitSessionQuery = { __typename?: 'Query', initSession?: { __typename?: 'OneTimeTokenType', user: { __typename?: 'UserType', firstName: string, lastName: string, email: string } } | null };

export type CheckSessionQueryVariables = Exact<{ [key: string]: never; }>;


export type CheckSessionQuery = { __typename?: 'Query', user?: { __typename?: 'UserType', email: string, firstName: string, lastName: string } | null };

export type LogoutQueryVariables = Exact<{ [key: string]: never; }>;


export type LogoutQuery = { __typename?: 'Query', logout?: { __typename?: 'LogoutType', success?: boolean | null } | null };

export type AskMutationVariables = Exact<{
  convId?: InputMaybe<Scalars['ID']['input']>;
  question: Scalars['String']['input'];
  page: Scalars['String']['input'];
  tabId: Scalars['ID']['input'];
}>;


export type AskMutation = { __typename?: 'Mutations', ask?: { __typename?: 'AskQuestion', response?: { __typename?: 'MessageType', id: any, text: string, isUser: boolean } | null } | null };

export type ConversationByTabIdQueryVariables = Exact<{
  tabId: Scalars['ID']['input'];
}>;


export type ConversationByTabIdQuery = { __typename?: 'Query', user?: { __typename?: 'UserType', conversationByTabId?: { __typename?: 'ConversationType', id: any, messages: Array<{ __typename?: 'MessageType', id: any, text: string, isUser: boolean }> } | null } | null };


export const InitSessionDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"InitSession"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"token"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"initSession"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"token"},"value":{"kind":"Variable","name":{"kind":"Name","value":"token"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"firstName"}},{"kind":"Field","name":{"kind":"Name","value":"lastName"}},{"kind":"Field","name":{"kind":"Name","value":"email"}}]}}]}}]}}]} as unknown as DocumentNode<InitSessionQuery, InitSessionQueryVariables>;
export const CheckSessionDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"CheckSession"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"email"}},{"kind":"Field","name":{"kind":"Name","value":"firstName"}},{"kind":"Field","name":{"kind":"Name","value":"lastName"}}]}}]}}]} as unknown as DocumentNode<CheckSessionQuery, CheckSessionQueryVariables>;
export const LogoutDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"Logout"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"logout"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"success"}}]}}]}}]} as unknown as DocumentNode<LogoutQuery, LogoutQueryVariables>;
export const AskDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"Ask"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"convId"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"question"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"page"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"tabId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"ask"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"conversationId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"convId"}}},{"kind":"Argument","name":{"kind":"Name","value":"question"},"value":{"kind":"Variable","name":{"kind":"Name","value":"question"}}},{"kind":"Argument","name":{"kind":"Name","value":"page"},"value":{"kind":"Variable","name":{"kind":"Name","value":"page"}}},{"kind":"Argument","name":{"kind":"Name","value":"tabId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"tabId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"response"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"text"}},{"kind":"Field","name":{"kind":"Name","value":"isUser"}}]}}]}}]}}]} as unknown as DocumentNode<AskMutation, AskMutationVariables>;
export const ConversationByTabIdDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"ConversationByTabId"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"tabId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"conversationByTabId"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"tabId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"tabId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"messages"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"text"}},{"kind":"Field","name":{"kind":"Name","value":"isUser"}}]}}]}}]}}]}}]} as unknown as DocumentNode<ConversationByTabIdQuery, ConversationByTabIdQueryVariables>;
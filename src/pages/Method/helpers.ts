import { ParameterValue } from "src/bindings";

export function getResultValue(value: ParameterValue) {
  if ("String" in value) {
    return value.String;
  }
  if ("Float" in value) {
    return value.Float;
  }
  if ("Bool" in value) {
    return value.Bool.toString();
  }
  return null;
}

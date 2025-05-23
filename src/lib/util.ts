import { join } from "path";
import { cwd } from "process";
export function pathFromProjectRoot(...args: string[]) {
  return join(cwd(), ...args);
}

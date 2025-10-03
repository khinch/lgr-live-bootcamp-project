import { v4 as uuidv4 } from "uuid";

export function getUuid(): string {
  return uuidv4();
}

export function getRandomEmail(domain: string): string {
  return `${getUuid()}@${domain}`;
}

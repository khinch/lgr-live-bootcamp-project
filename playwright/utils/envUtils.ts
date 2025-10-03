export function getEnv(name: string): string {
  const value = process.env[name];
  if (typeof value === "undefined") {
    throw new Error(`Environment variable ${name} is not set.`);
  }
  return value;
}

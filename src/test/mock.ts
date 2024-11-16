import MOCK_DATA from "./mock_data.json";

export let MOCKED = false;

/**
 * Setup mocks
 */
export function setupMocks() {
  // Check if the __TAURI_INTERNALS__ object exists/is readonly
  if (typeof (window as any).__TAURI_INTERNALS__ === "undefined") {
    MOCKED = true;
    mockInvoke();
  }
}

/**
 * Mock for the invoke function.
 */
export function mockInvoke() {
  (window as any).__TAURI_INTERNALS__ = {
    invoke: async (command: string, args: any) => {
      switch (command) {
        case "get_explorer_state":
          return Promise.resolve(MOCK_DATA.explorerState);
        default:
          console.warn(
            `invoke is not available. Command: ${command} Args: ${JSON.stringify(
              args,
            )}`,
          );
          return Promise.resolve(null);
      }
    },
  };
}

// import fs from "fs";
// const convertToJSON = (data) => {
//   return JSON.parse(JSON.parse(data));
// };
// const convertJSObjectToJSON = (data) => {
//   return JSON.stringify(data, null, 2);
// };
// const data = fs.readFileSync("./temp.txt", "utf8");
// const converted = convertToJSON(data);
// fs.writeFileSync("./converted.json", convertJSObjectToJSON(converted));

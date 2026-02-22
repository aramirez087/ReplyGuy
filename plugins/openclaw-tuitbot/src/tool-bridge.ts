/**
 * Converts MCP tools into OpenClaw tool registrations.
 *
 * Iterates the tools exposed by the MCP server, applies an optional
 * allowlist filter, and registers each as a native OpenClaw tool
 * prefixed with `tuitbot_`.
 */

import type { McpClient, McpTool } from "./mcp-client.js";

// ---------------------------------------------------------------------------
// OpenClaw API types (minimal surface used by the plugin)
// ---------------------------------------------------------------------------

export interface OpenClawToolRegistration {
  name: string;
  description: string;
  parameters: Record<string, unknown>;
  optional: boolean;
  execute: (args: Record<string, unknown>) => Promise<unknown>;
}

export interface OpenClawApi {
  registerTool(tool: OpenClawToolRegistration): void;
}

// ---------------------------------------------------------------------------
// Bridge
// ---------------------------------------------------------------------------

export interface BridgeOptions {
  /** MCP tool names to register. Empty or undefined = register all. */
  allowedTools?: string[];
}

/**
 * Bridge MCP tools into OpenClaw tool registrations.
 *
 * @returns The number of tools registered.
 */
export async function bridgeTools(
  client: McpClient,
  api: OpenClawApi,
  options: BridgeOptions = {},
): Promise<number> {
  const mcpTools = await client.listTools();
  const allowSet =
    options.allowedTools && options.allowedTools.length > 0
      ? new Set(options.allowedTools)
      : null;

  let count = 0;

  for (const tool of mcpTools) {
    if (allowSet && !allowSet.has(tool.name)) continue;

    const openclawName = `tuitbot_${tool.name}`;

    api.registerTool({
      name: openclawName,
      description: tool.description ?? `Tuitbot MCP tool: ${tool.name}`,
      parameters: tool.inputSchema,
      optional: true,
      execute: async (args: Record<string, unknown>) => {
        return client.callTool(tool.name, args);
      },
    });

    count++;
  }

  return count;
}

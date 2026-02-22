/**
 * MCP JSON-RPC client over stdio.
 *
 * Spawns `tuitbot mcp serve` as a child process and communicates via
 * newline-delimited JSON-RPC 2.0 over stdin/stdout.
 */

import { spawn, type ChildProcess } from "node:child_process";
import { randomUUID } from "node:crypto";
import { once, EventEmitter } from "node:events";
import { createInterface, type Interface as ReadlineInterface } from "node:readline";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface JsonRpcRequest {
  jsonrpc: "2.0";
  id: string;
  method: string;
  params?: Record<string, unknown>;
}

interface JsonRpcNotification {
  jsonrpc: "2.0";
  method: string;
  params?: Record<string, unknown>;
}

interface JsonRpcResponse {
  jsonrpc: "2.0";
  id: string;
  result?: unknown;
  error?: { code: number; message: string; data?: unknown };
}

export interface McpTool {
  name: string;
  description?: string;
  inputSchema: Record<string, unknown>;
}

export interface McpToolResult {
  content: Array<{ type: string; text: string }>;
  isError?: boolean;
}

interface PendingRequest {
  resolve: (value: unknown) => void;
  reject: (reason: Error) => void;
}

export interface McpClientOptions {
  /** Path to the tuitbot binary. Defaults to "tuitbot". */
  binaryPath?: string;
  /** Path to the tuitbot config file. */
  configPath?: string;
  /** Additional environment variables for the child process. */
  env?: Record<string, string>;
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

export class McpClient extends EventEmitter {
  private process: ChildProcess | null = null;
  private readline: ReadlineInterface | null = null;
  private pending = new Map<string, PendingRequest>();
  private started = false;

  constructor(private options: McpClientOptions = {}) {
    super();
  }

  /** Start the MCP server child process and perform the initialize handshake. */
  async start(): Promise<void> {
    if (this.started) return;

    const bin = this.options.binaryPath ?? "tuitbot";
    const args = ["mcp", "serve"];
    if (this.options.configPath) {
      args.unshift("--config", this.options.configPath);
    }

    const env: Record<string, string> = {
      ...process.env as Record<string, string>,
      OPENCLAW_PLUGIN: "tuitbot",
      ...this.options.env,
    };

    this.process = spawn(bin, args, {
      stdio: ["pipe", "pipe", "pipe"],
      env,
    });

    this.process.on("error", (err) => {
      this.emit("error", err);
    });

    this.process.on("exit", (code) => {
      this.started = false;
      // Reject all pending requests.
      for (const [id, pending] of this.pending) {
        pending.reject(new Error(`MCP process exited with code ${code}`));
        this.pending.delete(id);
      }
      this.emit("exit", code);
    });

    // Read newline-delimited JSON-RPC responses from stdout.
    this.readline = createInterface({ input: this.process.stdout! });
    this.readline.on("line", (line) => {
      this.handleLine(line);
    });

    // Perform initialize handshake.
    const initResult = await this.request("initialize", {
      protocolVersion: "2024-11-05",
      capabilities: {},
      clientInfo: { name: "openclaw-tuitbot-plugin", version: "0.1.0" },
    });

    // Send initialized notification.
    this.notify("notifications/initialized");

    this.started = true;
    this.emit("ready", initResult);
  }

  /** List all tools exposed by the MCP server. */
  async listTools(): Promise<McpTool[]> {
    const result = (await this.request("tools/list", {})) as {
      tools: McpTool[];
    };
    return result.tools;
  }

  /** Call an MCP tool by name with the given arguments. */
  async callTool(
    name: string,
    args: Record<string, unknown> = {},
  ): Promise<McpToolResult> {
    const result = (await this.request("tools/call", {
      name,
      arguments: args,
    })) as McpToolResult;
    return result;
  }

  /** Gracefully stop the MCP server child process. */
  async stop(): Promise<void> {
    if (!this.process) return;

    this.readline?.close();
    this.readline = null;

    const proc = this.process;
    this.process = null;
    this.started = false;

    // Give the process time to exit gracefully.
    const exitPromise = once(proc, "exit").catch(() => {});
    proc.kill("SIGTERM");

    const timeout = setTimeout(() => {
      proc.kill("SIGKILL");
    }, 5_000);

    await exitPromise;
    clearTimeout(timeout);
  }

  // -----------------------------------------------------------------------
  // Private
  // -----------------------------------------------------------------------

  private request(
    method: string,
    params?: Record<string, unknown>,
  ): Promise<unknown> {
    return new Promise((resolve, reject) => {
      if (!this.process?.stdin?.writable) {
        return reject(new Error("MCP process is not running"));
      }

      const id = randomUUID();
      const message: JsonRpcRequest = {
        jsonrpc: "2.0",
        id,
        method,
        ...(params && { params }),
      };

      this.pending.set(id, { resolve, reject });
      this.process.stdin.write(JSON.stringify(message) + "\n");
    });
  }

  private notify(
    method: string,
    params?: Record<string, unknown>,
  ): void {
    if (!this.process?.stdin?.writable) return;

    const message: JsonRpcNotification = {
      jsonrpc: "2.0",
      method,
      ...(params && { params }),
    };

    this.process.stdin.write(JSON.stringify(message) + "\n");
  }

  private handleLine(line: string): void {
    const trimmed = line.trim();
    if (!trimmed) return;

    let response: JsonRpcResponse;
    try {
      response = JSON.parse(trimmed) as JsonRpcResponse;
    } catch {
      // Not a valid JSON-RPC message — might be a log line, ignore.
      return;
    }

    if (!response.id) return; // Notification from server, ignore.

    const pending = this.pending.get(response.id);
    if (!pending) return;

    this.pending.delete(response.id);

    if (response.error) {
      pending.reject(
        new Error(
          `MCP error ${response.error.code}: ${response.error.message}`,
        ),
      );
    } else {
      pending.resolve(response.result);
    }
  }
}

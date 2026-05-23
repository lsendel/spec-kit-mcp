/**
 * Spec-Kit MCP Server — Pure Node.js Implementation
 *
 * Windows-compatible MCP server that wraps GitHub's spec-kit CLI via uvx.
 * Used as a fallback when the native Rust binary is not available (e.g. Windows).
 *
 * All 10 tools from the spec-kit workflow are supported:
 *   speckit_check, speckit_init, speckit_constitution, speckit_specify,
 *   speckit_plan, speckit_tasks, speckit_implement, speckit_clarify,
 *   speckit_analyze, speckit_checklist
 */

const { Server } = require('@modelcontextprotocol/sdk/server/index.js');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio.js');
const {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} = require('@modelcontextprotocol/sdk/types.js');
const tools = require('./tools.js');

const server = new Server(
  {
    name: 'spec-kit-mcp',
    version: '0.2.0-wrapper',
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

const toolRegistry = [
  {
    name: 'speckit_check',
    description: 'Validate that required tools are installed for spec-kit development (uv, python, git)',
    inputSchema: {
      type: 'object',
      properties: {
        check_speckit: { type: 'boolean', description: 'Check if spec-kit/speckit CLI is available' },
        check_git: { type: 'boolean', description: 'Check if git is installed' },
        check_ai_tools: { type: 'boolean', description: 'Check if AI coding tools are installed' },
      },
    },
    handler: tools.handleSpeckitCheck,
  },
  {
    name: 'speckit_init',
    description: 'Initialize a new spec-kit project with proper structure',
    inputSchema: {
      type: 'object',
      properties: {
        project_name: { type: 'string', description: 'Name of the project to initialize' },
        project_path: { type: 'string', description: 'Path where to initialize the project' },
        integration: { type: 'string', description: 'AI coding agent integration (e.g. copilot, claude)' },
      },
    },
    handler: tools.handleSpeckitInit,
  },
  {
    name: 'speckit_constitution',
    description: 'Create project governing principles and development standards',
    inputSchema: {
      type: 'object',
      properties: {
        principles: { type: 'string', description: 'Core principles guiding the project' },
        constraints: { type: 'string', description: 'Project constraints and requirements' },
        output_path: { type: 'string', description: 'Path for the constitution file' },
      },
    },
    handler: tools.handleSpeckitConstitution,
  },
  {
    name: 'speckit_specify',
    description: 'Define requirements and user stories (the "what")',
    inputSchema: {
      type: 'object',
      properties: {
        requirements: { type: 'string', description: 'Detailed requirements description' },
        user_stories: { type: 'string', description: 'User stories in structured format' },
        output_path: { type: 'string', description: 'Path for the specification file' },
      },
    },
    handler: tools.handleSpeckitSpecify,
  },
  {
    name: 'speckit_plan',
    description: 'Create a technical implementation plan (the "how")',
    inputSchema: {
      type: 'object',
      properties: {
        spec_file: { type: 'string', description: 'Path to the specification file' },
        tech_stack: { type: 'string', description: 'Technology stack to use' },
        output_path: { type: 'string', description: 'Path for the plan file' },
      },
    },
    handler: tools.handleSpeckitPlan,
  },
  {
    name: 'speckit_tasks',
    description: 'Generate actionable task lists from the plan',
    inputSchema: {
      type: 'object',
      properties: {
        plan_file: { type: 'string', description: 'Path to the plan file' },
        breakdown_level: { type: 'string', enum: ['low', 'medium', 'high'], description: 'Level of task breakdown detail' },
        output_path: { type: 'string', description: 'Path for the tasks file' },
      },
    },
    handler: tools.handleSpeckitTasks,
  },
  {
    name: 'speckit_implement',
    description: 'Execute implementation according to the task list',
    inputSchema: {
      type: 'object',
      properties: {
        task_file: { type: 'string', description: 'Path to the tasks file' },
        context: { type: 'string', description: 'Additional context for implementation' },
        output_dir: { type: 'string', description: 'Output directory for implementation' },
      },
    },
    handler: tools.handleSpeckitImplement,
  },
  {
    name: 'speckit_clarify',
    description: 'Request clarification on ambiguous requirements or specifications',
    inputSchema: {
      type: 'object',
      properties: {
        spec_file: { type: 'string', description: 'Path to the specification file' },
        questions: { type: 'string', description: 'Questions to clarify' },
      },
    },
    handler: tools.handleSpeckitClarify,
  },
  {
    name: 'speckit_analyze',
    description: 'Analyze code for quality, compliance, and technical debt',
    inputSchema: {
      type: 'object',
      properties: {
        target_path: { type: 'string', description: 'Path to code to analyze' },
        check_constitution: { type: 'boolean', description: 'Check against constitution' },
        output_format: { type: 'string', enum: ['markdown', 'json', 'text'], default: 'markdown', description: 'Output format' },
      },
    },
    handler: tools.handleSpeckitAnalyze,
  },
  {
    name: 'speckit_checklist',
    description: 'Generate review checklists to verify implementation completeness',
    inputSchema: {
      type: 'object',
      properties: {
        spec_file: { type: 'string', description: 'Path to the specification file' },
        task_file: { type: 'string', description: 'Path to the tasks file' },
        output_path: { type: 'string', description: 'Path for the checklist file' },
      },
    },
    handler: tools.handleSpeckitChecklist,
  },
];

server.setRequestHandler(ListToolsRequestSchema, async () => {
  return { tools: toolRegistry.map(({ handler, ...t }) => t) };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;
  const tool = toolRegistry.find((t) => t.name === name);

  if (!tool) {
    return {
      content: [{ type: 'text', text: `Unknown tool: ${name}` }],
      isError: true,
    };
  }

  try {
    const result = await tool.handler(args || {});
    return { content: [{ type: 'text', text: result }] };
  } catch (error) {
    return {
      content: [{ type: 'text', text: `Error: ${error.message}` }],
      isError: true,
    };
  }
});

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  process.stderr.write('spec-kit-mcp [node] running\n');
}

main().catch((error) => {
  process.stderr.write(`Failed to start server: ${error.message}\n`);
  process.exit(1);
});

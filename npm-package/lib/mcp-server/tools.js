/**
 * Spec-Kit MCP Tools — Pure Node.js Implementation
 *
 * Each tool wraps the spec-kit CLI executed via:
 *   uvx --from git+https://github.com/github/spec-kit.git <command>
 */

const { exec } = require('child_process');
const { promisify } = require('util');
const fs = require('fs').promises;
const path = require('path');

const execAsync = promisify(exec);

const SPEC_KIT_REPO = 'git+https://github.com/github/spec-kit.git';

async function runUvx(args, timeoutMs = 120000) {
  const cmd = `uvx --from ${SPEC_KIT_REPO} ${args}`;
  try {
    const { stdout, stderr } = await execAsync(cmd, { timeout: timeoutMs });
    return stdout || stderr;
  } catch (error) {
    if (error.stdout) return error.stdout;
    if (error.stderr) return error.stderr;
    throw error;
  }
}

async function ensureDir(dirPath) {
  if (dirPath && dirPath !== '.') {
    await fs.mkdir(dirPath, { recursive: true });
  }
}

// ── speckit_check ──────────────────────────────────────────────

async function handleSpeckitCheck(args = {}) {
  const results = [];

  try {
    await execAsync('python --version');
    results.push('✓ Python installed');
  } catch {
    results.push('✗ Python NOT installed (required: 3.11+)');
  }

  try {
    await execAsync('uv --version');
    results.push('✓ uv installed');
  } catch {
    results.push('✗ uv NOT installed (required)');
  }

  if (args.check_git !== false) {
    try {
      await execAsync('git --version');
      results.push('✓ git installed');
    } catch {
      results.push('✗ git NOT installed');
    }
  }

  if (args.check_speckit !== false) {
    try {
      await runUvx('specify --version', 30000);
      results.push('✓ spec-kit CLI available via uvx');
    } catch {
      results.push('✗ spec-kit CLI unavailable');
    }
  }

  if (args.check_ai_tools) {
    const aiTools = ['claude', 'codex', 'opencode', 'gemini', 'cursor'];
    for (const tool of aiTools) {
      try {
        await execAsync(`${tool} --version`);
        results.push(`✓ ${tool} available`);
      } catch {
        results.push(`- ${tool} not detected`);
      }
    }
  }

  return results.join('\n');
}

// ── speckit_init ───────────────────────────────────────────────

async function handleSpeckitInit(args = {}) {
  const { project_name, project_path, integration } = args;
  if (!project_name) throw new Error('project_name is required');

  let cmd = `specify init ${project_name}`;
  if (project_path && project_path !== '.') cmd += ` --dir ${project_path}`;
  if (integration) cmd += ` --integration ${integration}`;

  return await runUvx(cmd);
}

// ── speckit_constitution ───────────────────────────────────────

async function handleSpeckitConstitution(args = {}) {
  const { principles, constraints, output_path } = args;
  if (!principles) throw new Error('principles is required');

  const content = `# Project Constitution\n\n## Principles\n${principles}\n\n## Constraints\n${constraints || 'No specific constraints defined.'}\n`;

  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, content, 'utf-8');
    return `Constitution written to ${output_path}`;
  }
  return content;
}

// ── speckit_specify ────────────────────────────────────────────

async function handleSpeckitSpecify(args = {}) {
  const { requirements, user_stories, output_path } = args;
  if (!requirements) throw new Error('requirements is required');

  const content = `# Specification\n\n## Requirements\n${requirements}\n\n## User Stories\n${user_stories || 'No user stories provided.'}\n`;

  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, content, 'utf-8');
    return `Specification written to ${output_path}`;
  }
  return content;
}

// ── speckit_plan ───────────────────────────────────────────────

async function handleSpeckitPlan(args = {}) {
  const { spec_file, tech_stack, output_path } = args;
  if (!spec_file) throw new Error('spec_file is required');

  let cmd = `specify plan --spec ${spec_file}`;
  if (tech_stack) cmd += ` --tech-stack "${tech_stack}"`;

  const result = await runUvx(cmd);
  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, result, 'utf-8');
    return `Plan written to ${output_path}\n\n${result}`;
  }
  return result;
}

// ── speckit_tasks ──────────────────────────────────────────────

async function handleSpeckitTasks(args = {}) {
  const { plan_file, breakdown_level, output_path } = args;
  if (!plan_file) throw new Error('plan_file is required');

  let cmd = `specify tasks --plan ${plan_file}`;
  if (breakdown_level) cmd += ` --level ${breakdown_level}`;

  const result = await runUvx(cmd);
  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, result, 'utf-8');
    return `Tasks written to ${output_path}\n\n${result}`;
  }
  return result;
}

// ── speckit_implement ──────────────────────────────────────────

async function handleSpeckitImplement(args = {}) {
  const { task_file, context, output_dir } = args;
  if (!task_file) throw new Error('task_file is required');

  let cmd = `specify implement --tasks ${task_file}`;
  if (context) cmd += ` --context "${context}"`;
  if (output_dir) cmd += ` --output-dir ${output_dir}`;

  return await runUvx(cmd);
}

// ── speckit_clarify ────────────────────────────────────────────

async function handleSpeckitClarify(args = {}) {
  const { spec_file, questions } = args;
  if (!spec_file) throw new Error('spec_file is required');

  let cmd = `specify clarify --spec ${spec_file}`;
  if (questions) cmd += ` --questions "${questions}"`;

  return await runUvx(cmd);
}

// ── speckit_analyze ────────────────────────────────────────────

async function handleSpeckitAnalyze(args = {}) {
  const { target_path, check_constitution, output_format = 'markdown' } = args;
  if (!target_path) throw new Error('target_path is required');

  let cmd = `specify analyze ${target_path}`;
  if (check_constitution) cmd += ' --check-constitution';
  cmd += ` --format ${output_format}`;

  return await runUvx(cmd);
}

// ── speckit_checklist ──────────────────────────────────────────

async function handleSpeckitChecklist(args = {}) {
  const { spec_file, task_file, output_path } = args;
  if (!spec_file) throw new Error('spec_file is required');

  let cmd = `specify checklist --spec ${spec_file}`;
  if (task_file) cmd += ` --tasks ${task_file}`;

  const result = await runUvx(cmd);
  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, result, 'utf-8');
    return `Checklist written to ${output_path}\n\n${result}`;
  }
  return result;
}

module.exports = {
  handleSpeckitCheck,
  handleSpeckitInit,
  handleSpeckitConstitution,
  handleSpeckitSpecify,
  handleSpeckitPlan,
  handleSpeckitTasks,
  handleSpeckitImplement,
  handleSpeckitClarify,
  handleSpeckitAnalyze,
  handleSpeckitChecklist,
};

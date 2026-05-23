/**
 * Spec-Kit MCP Tools — Pure Node.js Implementation
 *
 * Each tool wraps the spec-kit CLI executed via:
 *   uvx --from git+https://github.com/github/spec-kit.git <command>
 *
 * Uses spawn() with argv arrays to prevent command injection.
 * Non-zero exits from uvx are propagated as MCP tool errors (isError: true).
 */

const { spawn } = require('child_process');
const fs = require('fs').promises;
const path = require('path');

const SPEC_KIT_REPO = 'git+https://github.com/github/spec-kit.git';

/**
 * Run a uvx command with argv array (no shell, no injection).
 * Throws on non-zero exit so the MCP handler correctly sets isError: true.
 */
function runUvx(argv, timeoutMs = 120000) {
  return new Promise((resolve, reject) => {
    const child = spawn('uvx', ['--from', SPEC_KIT_REPO, ...argv], {
      timeout: timeoutMs,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });

    let stdout = '';
    let stderr = '';

    child.stdout.on('data', (d) => { stdout += d.toString(); });
    child.stderr.on('data', (d) => { stderr += d.toString(); });

    child.on('error', (err) => {
      reject(new Error(`Failed to spawn uvx: ${err.message}`));
    });

    child.on('close', (code) => {
      if (code === 0) {
        resolve(stdout || stderr);
      } else {
        reject(new Error(
          `uvx exited with code ${code}\n${stderr || stdout || ''}`.trim()
        ));
      }
    });
  });
}

async function ensureDir(dirPath) {
  if (dirPath && dirPath !== '.') {
    await fs.mkdir(dirPath, { recursive: true });
  }
}

function checkCommand(cmd, args, timeoutMs = 10000) {
  return new Promise((resolve, reject) => {
    const child = spawn(cmd, args, {
      timeout: timeoutMs,
      stdio: 'ignore',
      windowsHide: true,
    });
    child.on('error', reject);
    child.on('close', (code) => {
      code === 0 ? resolve() : reject(new Error(`exit ${code}`));
    });
  });
}

// ── speckit_check ──────────────────────────────────────────────

async function handleSpeckitCheck(args = {}) {
  const results = [];

  try {
    await checkCommand('python', ['--version']);
    results.push('✓ Python installed');
  } catch {
    results.push('✗ Python NOT installed (required: 3.11+)');
  }

  try {
    await checkCommand('uv', ['--version']);
    results.push('✓ uv installed');
  } catch {
    results.push('✗ uv NOT installed (required)');
  }

  if (args.check_git !== false) {
    try {
      await checkCommand('git', ['--version']);
      results.push('✓ git installed');
    } catch {
      results.push('✗ git NOT installed');
    }
  }

  if (args.check_speckit !== false) {
    try {
      await runUvx(['specify', '--version'], 30000);
      results.push('✓ spec-kit CLI available via uvx');
    } catch {
      results.push('✗ spec-kit CLI unavailable');
    }
  }

  return results.join('\n');
}

// ── speckit_init ───────────────────────────────────────────────

async function handleSpeckitInit(args = {}) {
  const { project_name, project_path, integration } = args;
  if (!project_name) throw new Error('project_name is required');

  // Use --here for current directory, positional <project_name> otherwise.
  // There is no --dir flag in spec-kit init.
  const argv = ['specify', 'init'];
  if (project_path === '.' || project_path === './') {
    argv.push('--here');
  } else {
    argv.push(project_name);
  }
  if (integration) argv.push('--integration', integration);

  return await runUvx(argv);
}

// ── speckit_constitution ───────────────────────────────────────

async function handleSpeckitConstitution(args = {}) {
  const { principles, constraints, output_path } = args;
  if (!principles) throw new Error('principles is required');

  const content =
    '# Project Constitution\n\n' +
    '## Principles\n' + principles + '\n\n' +
    '## Constraints\n' + (constraints || 'No specific constraints defined.') + '\n';

  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, content, 'utf-8');
    return 'Constitution written to ' + output_path;
  }
  return content;
}

// ── speckit_specify ────────────────────────────────────────────

async function handleSpeckitSpecify(args = {}) {
  const { requirements, user_stories, output_path } = args;
  if (!requirements) throw new Error('requirements is required');

  const content =
    '# Specification\n\n' +
    '## Requirements\n' + requirements + '\n\n' +
    '## User Stories\n' + (user_stories || 'No user stories provided.') + '\n';

  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, content, 'utf-8');
    return 'Specification written to ' + output_path;
  }
  return content;
}

// ── speckit_plan ───────────────────────────────────────────────

async function handleSpeckitPlan(args = {}) {
  const { spec_file, tech_stack, output_path } = args;
  if (!spec_file) throw new Error('spec_file is required');

  const argv = ['specify', 'plan', '--spec', spec_file];
  if (tech_stack) argv.push('--tech-stack', tech_stack);

  const result = await runUvx(argv);
  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, result, 'utf-8');
    return 'Plan written to ' + output_path + '\n\n' + result;
  }
  return result;
}

// ── speckit_tasks ──────────────────────────────────────────────

async function handleSpeckitTasks(args = {}) {
  const { plan_file, breakdown_level, output_path } = args;
  if (!plan_file) throw new Error('plan_file is required');

  const argv = ['specify', 'tasks', '--plan', plan_file];
  if (breakdown_level) argv.push('--level', breakdown_level);

  const result = await runUvx(argv);
  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, result, 'utf-8');
    return 'Tasks written to ' + output_path + '\n\n' + result;
  }
  return result;
}

// ── speckit_implement ──────────────────────────────────────────

async function handleSpeckitImplement(args = {}) {
  const { task_file, context, output_dir } = args;
  if (!task_file) throw new Error('task_file is required');

  const argv = ['specify', 'implement', '--tasks', task_file];
  if (context) argv.push('--context', context);
  if (output_dir) argv.push('--output-dir', output_dir);

  return await runUvx(argv);
}

// ── speckit_clarify ────────────────────────────────────────────

async function handleSpeckitClarify(args = {}) {
  const { spec_file, questions } = args;
  if (!spec_file) throw new Error('spec_file is required');

  const argv = ['specify', 'clarify', '--spec', spec_file];
  if (questions) argv.push('--questions', questions);

  return await runUvx(argv);
}

// ── speckit_analyze ────────────────────────────────────────────

async function handleSpeckitAnalyze(args = {}) {
  const { target_path, check_constitution, output_format } = args;
  if (!target_path) throw new Error('target_path is required');

  const argv = ['specify', 'analyze', target_path];
  if (check_constitution) argv.push('--check-constitution');
  argv.push('--format', output_format || 'markdown');

  return await runUvx(argv);
}

// ── speckit_checklist ──────────────────────────────────────────

async function handleSpeckitChecklist(args = {}) {
  const { spec_file, task_file, output_path } = args;
  if (!spec_file) throw new Error('spec_file is required');

  const argv = ['specify', 'checklist', '--spec', spec_file];
  if (task_file) argv.push('--tasks', task_file);

  const result = await runUvx(argv);
  if (output_path) {
    await ensureDir(path.dirname(output_path));
    await fs.writeFile(output_path, result, 'utf-8');
    return 'Checklist written to ' + output_path + '\n\n' + result;
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

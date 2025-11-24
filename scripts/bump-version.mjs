import {readFile, writeFile} from 'node:fs/promises';
import {dirname, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const newVersion = process.argv[2];

if (!newVersion) {
  console.error('❌ Usage: node scripts/bump-version.mjs <new-version>');
  process.exit(1);
}

// Validate version format (SemVer)
if (!/^\d+\.\d+\.\d+$/.test(newVersion)) {
  console.error('❌ Error: Version must be in SemVer format (e.g., 1.2.3)');
  process.exit(1);
}

const files = [
  {
    path: 'package.json',
    update: (content) => {
      const json = JSON.parse(content);
      json.version = newVersion;
      return JSON.stringify(json, null, 2) + '\n';
    },
  },
  {
    path: 'src-tauri/tauri.conf.json',
    update: (content) => {
      const json = JSON.parse(content);
      json.version = newVersion;
      return JSON.stringify(json, null, 2) + '\n';
    },
  },
  {
    path: 'src-tauri/Cargo.toml',
    update: (content) => {
      return content.replace(/^version = ".*"/m, `version = "${newVersion}"`);
    },
  },
  {
    path: 'snapcraft.yaml',
    update: (content) => {
      return content.replace(/^version: '.*'/m, `version: '${newVersion}'`);
    },
  },
];

console.log(`🚀 Bumping version to ${newVersion}...`);

async function bump() {
  for (const file of files) {
    const filePath = resolve(__dirname, '..', file.path);
    try {
      const content = await readFile(filePath, 'utf8');
      const newContent = file.update(content);
      await writeFile(filePath, newContent);
      console.log(`✅ Updated ${file.path}`);
    } catch (error) {
      if (error.code === 'ENOENT') {
        console.warn(`⚠️  Warning: ${file.path} not found`);
      } else {
        console.error(`❌ Error updating ${file.path}:`, error);
        process.exit(1);
      }
    }
  }
  console.log('✨ Version bump complete.');
}

bump();

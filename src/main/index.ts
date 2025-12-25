import { app, BrowserWindow, ipcMain, dialog, Menu, shell } from 'electron'
import { readFile, writeFile, readdir, stat, mkdir, rename, rm, access } from 'fs/promises'
import { constants } from 'fs'
import { spawn } from 'child_process'
import { join, basename } from 'path'
import { getLSPClient, destroyLSPClient } from './lsp-client'

let mainWindow: BrowserWindow | null = null

function createWindow(): void {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    minWidth: 800,
    minHeight: 600,
    title: 'Qalam - Tarqeem Editor',
    webPreferences: {
      preload: join(__dirname, '../preload/index.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: false
    }
  })

  // Load the renderer
  if (process.env.NODE_ENV === 'development') {
    mainWindow.loadURL('http://localhost:5173')
    mainWindow.webContents.openDevTools()
  } else {
    mainWindow.loadFile(join(__dirname, '../renderer/index.html'))
  }

  mainWindow.on('closed', () => {
    mainWindow = null
  })
}

// Create Arabic application menu
function createMenu(): void {
  const template: Electron.MenuItemConstructorOptions[] = [
    {
      label: 'ملف',
      submenu: [
        {
          label: 'فتح...',
          accelerator: 'CmdOrCtrl+O',
          click: () => mainWindow?.webContents.send('menu:open')
        },
        {
          label: 'حفظ',
          accelerator: 'CmdOrCtrl+S',
          click: () => mainWindow?.webContents.send('menu:save')
        },
        {
          label: 'حفظ باسم...',
          accelerator: 'CmdOrCtrl+Shift+S',
          click: () => mainWindow?.webContents.send('menu:save-as')
        },
        { type: 'separator' },
        {
          label: 'خروج',
          accelerator: process.platform === 'darwin' ? 'Cmd+Q' : 'Alt+F4',
          role: 'quit'
        }
      ]
    },
    {
      label: 'تحرير',
      submenu: [
        { label: 'تراجع', accelerator: 'CmdOrCtrl+Z', role: 'undo' },
        { label: 'إعادة', accelerator: 'CmdOrCtrl+Shift+Z', role: 'redo' },
        { type: 'separator' },
        { label: 'قص', accelerator: 'CmdOrCtrl+X', role: 'cut' },
        { label: 'نسخ', accelerator: 'CmdOrCtrl+C', role: 'copy' },
        { label: 'لصق', accelerator: 'CmdOrCtrl+V', role: 'paste' },
        { label: 'تحديد الكل', accelerator: 'CmdOrCtrl+A', role: 'selectAll' }
      ]
    },
    {
      label: 'بناء',
      submenu: [
        {
          label: 'ترجمة',
          accelerator: 'CmdOrCtrl+B',
          click: () => mainWindow?.webContents.send('menu:compile')
        },
        {
          label: 'تشغيل',
          accelerator: 'CmdOrCtrl+R',
          click: () => mainWindow?.webContents.send('menu:run')
        }
      ]
    },
    {
      label: 'عرض',
      submenu: [
        { label: 'تكبير الخط', accelerator: 'CmdOrCtrl+Plus', role: 'zoomIn' },
        { label: 'تصغير الخط', accelerator: 'CmdOrCtrl+-', role: 'zoomOut' },
        { label: 'حجم افتراضي', accelerator: 'CmdOrCtrl+0', role: 'resetZoom' },
        { type: 'separator' },
        { label: 'ملء الشاشة', accelerator: 'F11', role: 'togglefullscreen' }
      ]
    }
  ]

  // Add macOS app menu
  if (process.platform === 'darwin') {
    template.unshift({
      label: app.name,
      submenu: [
        { label: 'عن قلم', role: 'about' },
        { type: 'separator' },
        { label: 'إخفاء', role: 'hide' },
        { label: 'إخفاء الآخرين', role: 'hideOthers' },
        { label: 'إظهار الكل', role: 'unhide' },
        { type: 'separator' },
        { label: 'خروج', role: 'quit' }
      ]
    })
  }

  const menu = Menu.buildFromTemplate(template)
  Menu.setApplicationMenu(menu)
}

// IPC Handlers for file operations
ipcMain.handle('file:open', async () => {
  const result = await dialog.showOpenDialog({
    title: 'فتح ملف ترقيم',
    filters: [
      { name: 'ملفات ترقيم', extensions: ['ترقيم', 'trq'] },
      { name: 'جميع الملفات', extensions: ['*'] }
    ],
    properties: ['openFile']
  })

  if (result.canceled || !result.filePaths[0]) {
    return null
  }

  try {
    const content = await readFile(result.filePaths[0], 'utf-8')
    return { path: result.filePaths[0], content }
  } catch (error) {
    return { error: `Failed to read file: ${error}` }
  }
})

// Read file by path (for recent files)
ipcMain.handle('file:read', async (_, filePath: string) => {
  try {
    const content = await readFile(filePath, 'utf-8')
    return { content }
  } catch (error) {
    return null
  }
})

// Open folder dialog
ipcMain.handle('folder:open', async () => {
  const result = await dialog.showOpenDialog({
    title: 'فتح مجلد',
    properties: ['openDirectory']
  })

  if (result.canceled || !result.filePaths[0]) {
    return null
  }

  return {
    path: result.filePaths[0],
    name: basename(result.filePaths[0])
  }
})

// Read directory contents
ipcMain.handle('folder:read', async (_, dirPath: string) => {
  try {
    const entries = await readdir(dirPath, { withFileTypes: true })
    const items = await Promise.all(
      entries
        .filter(entry => !entry.name.startsWith('.')) // Hide hidden files
        .map(async (entry) => {
          const fullPath = join(dirPath, entry.name)
          return {
            name: entry.name,
            path: fullPath,
            type: entry.isDirectory() ? 'directory' : 'file'
          }
        })
    )

    // Sort: directories first, then files, alphabetically
    items.sort((a, b) => {
      if (a.type === b.type) {
        return a.name.localeCompare(b.name, 'ar')
      }
      return a.type === 'directory' ? -1 : 1
    })

    return items
  } catch (error) {
    return []
  }
})

// Create new file
ipcMain.handle('folder:createFile', async (_, filePath: string) => {
  try {
    await writeFile(filePath, '', 'utf-8')
    return { success: true }
  } catch (error) {
    return { error: `Failed to create file: ${error}` }
  }
})

// Create new folder
ipcMain.handle('folder:createFolder', async (_, folderPath: string) => {
  try {
    await mkdir(folderPath)
    return { success: true }
  } catch (error) {
    return { error: `Failed to create folder: ${error}` }
  }
})

// Rename file or folder
ipcMain.handle('folder:rename', async (_, oldPath: string, newPath: string) => {
  try {
    await rename(oldPath, newPath)
    return { success: true }
  } catch (error) {
    return { error: `Failed to rename: ${error}` }
  }
})

// Delete file or folder
ipcMain.handle('folder:delete', async (_, targetPath: string) => {
  try {
    await rm(targetPath, { recursive: true })
    return { success: true }
  } catch (error) {
    return { error: `Failed to delete: ${error}` }
  }
})

// Reveal in file explorer
ipcMain.handle('folder:reveal', async (_, targetPath: string) => {
  shell.showItemInFolder(targetPath)
  return { success: true }
})

// Project file name constant
const PROJECT_FILE_NAME = 'ترقيم.حزمة'

// Check if project file exists in folder
ipcMain.handle('project:exists', async (_, folderPath: string) => {
  try {
    const projectFilePath = join(folderPath, PROJECT_FILE_NAME)
    await access(projectFilePath, constants.F_OK)
    return { exists: true }
  } catch {
    return { exists: false }
  }
})

// Read project manifest
ipcMain.handle('project:read', async (_, folderPath: string) => {
  try {
    const projectFilePath = join(folderPath, PROJECT_FILE_NAME)
    const content = await readFile(projectFilePath, 'utf-8')
    const manifest = JSON.parse(content)
    return { success: true, manifest }
  } catch (error) {
    return { success: false, error: `فشل في قراءة ملف المشروع: ${error}` }
  }
})

// Write project manifest
ipcMain.handle('project:write', async (_, folderPath: string, manifest: unknown) => {
  try {
    const projectFilePath = join(folderPath, PROJECT_FILE_NAME)
    const content = JSON.stringify(manifest, null, 2)
    await writeFile(projectFilePath, content, 'utf-8')
    return { success: true }
  } catch (error) {
    return { success: false, error: `فشل في كتابة ملف المشروع: ${error}` }
  }
})

// Initialize new project with default manifest
ipcMain.handle('project:init', async (_, folderPath: string, projectName: string) => {
  try {
    const projectFilePath = join(folderPath, PROJECT_FILE_NAME)

    // Create default manifest with Arabic keys
    const defaultManifest = {
      'الاسم': projectName,
      'الإصدار': '1.0.0',
      'نقطة_البداية': 'main.ترقيم',
      'مجلد_الإخراج': 'build/',
      'إعدادات_المترجم': {
        'تحسين': 'أساسي',
        'وضع_التنقيح': true,
        'تحذيرات_كأخطاء': false,
        'مستوى_التحذيرات': 'أساسي'
      }
    }

    const content = JSON.stringify(defaultManifest, null, 2)
    await writeFile(projectFilePath, content, 'utf-8')
    return { success: true }
  } catch (error) {
    return { success: false, error: `فشل في تهيئة المشروع: ${error}` }
  }
})

ipcMain.handle('file:save', async (_, { path, content }: { path: string; content: string }) => {
  try {
    await writeFile(path, content, 'utf-8')
    return { success: true }
  } catch (error) {
    return { error: `Failed to save file: ${error}` }
  }
})

ipcMain.handle('file:save-as', async (_, content: string) => {
  const result = await dialog.showSaveDialog({
    title: 'حفظ الملف',
    defaultPath: 'untitled.ترقيم',
    filters: [
      { name: 'ملفات ترقيم', extensions: ['ترقيم'] },
      { name: 'جميع الملفات', extensions: ['*'] }
    ]
  })

  if (result.canceled || !result.filePath) {
    return null
  }

  try {
    await writeFile(result.filePath, content, 'utf-8')
    return { path: result.filePath }
  } catch (error) {
    return { error: `Failed to save file: ${error}` }
  }
})

// Compiler integration
ipcMain.handle('compiler:compile', async (event, filePath: string) => {
  return new Promise((resolve) => {
    const output: string[] = []
    const errors: string[] = []

    const proc = spawn('tarqeem', ['compile', filePath], {
      cwd: filePath.substring(0, filePath.lastIndexOf('/'))
    })

    proc.stdout.on('data', (data) => {
      const text = data.toString()
      output.push(text)
      event.sender.send('compiler:stdout', text)
    })

    proc.stderr.on('data', (data) => {
      const text = data.toString()
      errors.push(text)
      event.sender.send('compiler:stderr', text)
    })

    proc.on('error', (err) => {
      resolve({
        success: false,
        output: output.join(''),
        errors: `Failed to start compiler: ${err.message}`,
        exitCode: -1
      })
    })

    proc.on('close', (code) => {
      resolve({
        success: code === 0,
        output: output.join(''),
        errors: errors.join(''),
        exitCode: code
      })
    })
  })
})

ipcMain.handle('compiler:run', async (event, filePath: string) => {
  return new Promise((resolve) => {
    const output: string[] = []
    const errors: string[] = []

    const proc = spawn('tarqeem', ['run', filePath], {
      cwd: filePath.substring(0, filePath.lastIndexOf('/'))
    })

    proc.stdout.on('data', (data) => {
      const text = data.toString()
      output.push(text)
      event.sender.send('compiler:stdout', text)
    })

    proc.stderr.on('data', (data) => {
      const text = data.toString()
      errors.push(text)
      event.sender.send('compiler:stderr', text)
    })

    proc.on('error', (err) => {
      resolve({
        success: false,
        output: output.join(''),
        errors: `Failed to start: ${err.message}`,
        exitCode: -1
      })
    })

    proc.on('close', (code) => {
      resolve({
        success: code === 0,
        output: output.join(''),
        errors: errors.join(''),
        exitCode: code
      })
    })
  })
})

// LSP Integration
ipcMain.handle('lsp:start', async (event, workspacePath: string) => {
  try {
    const lspClient = getLSPClient()

    // Set up notification forwarding to renderer
    lspClient.on('diagnostics', (params) => {
      event.sender.send('lsp:diagnostics', params)
    })

    lspClient.on('log', (params) => {
      event.sender.send('lsp:log', params)
    })

    lspClient.on('showMessage', (params) => {
      event.sender.send('lsp:showMessage', params)
    })

    lspClient.on('error', (err) => {
      event.sender.send('lsp:error', { message: err.message })
    })

    lspClient.on('close', (code) => {
      event.sender.send('lsp:close', { code })
    })

    const result = await lspClient.start(workspacePath)
    return { success: true, capabilities: result.capabilities }
  } catch (error) {
    return { success: false, error: (error as Error).message }
  }
})

ipcMain.handle('lsp:stop', async () => {
  try {
    destroyLSPClient()
    return { success: true }
  } catch (error) {
    return { success: false, error: (error as Error).message }
  }
})

ipcMain.handle('lsp:request', async (_, method: string, params: unknown) => {
  try {
    const lspClient = getLSPClient()
    if (!lspClient.isRunning()) {
      return { success: false, error: 'LSP server not running' }
    }
    const result = await lspClient.request(method, params)
    return { success: true, result }
  } catch (error) {
    return { success: false, error: (error as Error).message }
  }
})

ipcMain.handle('lsp:notify', async (_, method: string, params: unknown) => {
  try {
    const lspClient = getLSPClient()
    if (!lspClient.isRunning()) {
      return { success: false, error: 'LSP server not running' }
    }
    lspClient.notify(method, params)
    return { success: true }
  } catch (error) {
    return { success: false, error: (error as Error).message }
  }
})

ipcMain.handle('lsp:isRunning', async () => {
  const lspClient = getLSPClient()
  return { running: lspClient.isRunning() }
})

// App lifecycle
app.whenReady().then(() => {
  createMenu()
  createWindow()

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow()
    }
  })
})

app.on('window-all-closed', () => {
  // Clean up LSP client before quitting
  destroyLSPClient()

  if (process.platform !== 'darwin') {
    app.quit()
  }
})

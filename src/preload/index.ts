import { contextBridge, ipcRenderer } from 'electron'

// Types for the exposed API
export interface FileData {
  path: string
  content: string
}

export interface FileError {
  error: string
}

export interface SaveResult {
  success?: boolean
  path?: string
  error?: string
}

export interface CompilerResult {
  success: boolean
  output: string
  errors: string
  exitCode: number
}

// LSP Types
export interface LSPStartResult {
  success: boolean
  capabilities?: unknown
  error?: string
}

export interface LSPRequestResult {
  success: boolean
  result?: unknown
  error?: string
}

export interface LSPDiagnostic {
  range: {
    start: { line: number; character: number }
    end: { line: number; character: number }
  }
  message: string
  severity?: number
  source?: string
}

export interface LSPDiagnosticsParams {
  uri: string
  diagnostics: LSPDiagnostic[]
}

// Expose protected methods to the renderer
contextBridge.exposeInMainWorld('qalam', {
  // File operations
  file: {
    open: (): Promise<FileData | FileError | null> =>
      ipcRenderer.invoke('file:open'),

    read: (path: string): Promise<{ content: string } | null> =>
      ipcRenderer.invoke('file:read', path),

    save: (path: string, content: string): Promise<SaveResult> =>
      ipcRenderer.invoke('file:save', { path, content }),

    saveAs: (content: string): Promise<SaveResult | null> =>
      ipcRenderer.invoke('file:save-as', content)
  },

  // Folder operations
  folder: {
    open: (): Promise<{ path: string; name: string } | null> =>
      ipcRenderer.invoke('folder:open'),

    read: (path: string): Promise<Array<{ name: string; path: string; type: 'file' | 'directory' }>> =>
      ipcRenderer.invoke('folder:read', path),

    createFile: (path: string): Promise<{ success?: boolean; error?: string }> =>
      ipcRenderer.invoke('folder:createFile', path),

    createFolder: (path: string): Promise<{ success?: boolean; error?: string }> =>
      ipcRenderer.invoke('folder:createFolder', path),

    rename: (oldPath: string, newPath: string): Promise<{ success?: boolean; error?: string }> =>
      ipcRenderer.invoke('folder:rename', oldPath, newPath),

    delete: (path: string): Promise<{ success?: boolean; error?: string }> =>
      ipcRenderer.invoke('folder:delete', path),

    reveal: (path: string): Promise<{ success: boolean }> =>
      ipcRenderer.invoke('folder:reveal', path)
  },

  // Project operations
  project: {
    exists: (folderPath: string): Promise<{ exists: boolean }> =>
      ipcRenderer.invoke('project:exists', folderPath),

    read: (folderPath: string): Promise<{ success: boolean; manifest?: unknown; error?: string }> =>
      ipcRenderer.invoke('project:read', folderPath),

    write: (folderPath: string, manifest: unknown): Promise<{ success: boolean; error?: string }> =>
      ipcRenderer.invoke('project:write', folderPath, manifest),

    init: (folderPath: string, projectName: string): Promise<{ success: boolean; error?: string }> =>
      ipcRenderer.invoke('project:init', folderPath, projectName)
  },

  // Compiler operations
  compiler: {
    compile: (filePath: string): Promise<CompilerResult> =>
      ipcRenderer.invoke('compiler:compile', filePath),

    run: (filePath: string): Promise<CompilerResult> =>
      ipcRenderer.invoke('compiler:run', filePath),

    onStdout: (callback: (output: string) => void): void => {
      ipcRenderer.on('compiler:stdout', (_, output) => callback(output))
    },

    onStderr: (callback: (error: string) => void): void => {
      ipcRenderer.on('compiler:stderr', (_, error) => callback(error))
    },

    removeListeners: (): void => {
      ipcRenderer.removeAllListeners('compiler:stdout')
      ipcRenderer.removeAllListeners('compiler:stderr')
    }
  },

  // Menu event listeners
  menu: {
    onOpen: (callback: () => void): (() => void) => {
      ipcRenderer.on('menu:open', callback)
      return () => ipcRenderer.removeListener('menu:open', callback)
    },
    onSave: (callback: () => void): (() => void) => {
      ipcRenderer.on('menu:save', callback)
      return () => ipcRenderer.removeListener('menu:save', callback)
    },
    onSaveAs: (callback: () => void): (() => void) => {
      ipcRenderer.on('menu:save-as', callback)
      return () => ipcRenderer.removeListener('menu:save-as', callback)
    },
    onCompile: (callback: () => void): (() => void) => {
      ipcRenderer.on('menu:compile', callback)
      return () => ipcRenderer.removeListener('menu:compile', callback)
    },
    onRun: (callback: () => void): (() => void) => {
      ipcRenderer.on('menu:run', callback)
      return () => ipcRenderer.removeListener('menu:run', callback)
    }
  },

  // LSP (Language Server Protocol) operations
  lsp: {
    start: (workspacePath: string): Promise<LSPStartResult> =>
      ipcRenderer.invoke('lsp:start', workspacePath),

    stop: (): Promise<{ success: boolean; error?: string }> =>
      ipcRenderer.invoke('lsp:stop'),

    request: (method: string, params: unknown): Promise<LSPRequestResult> =>
      ipcRenderer.invoke('lsp:request', method, params),

    notify: (method: string, params: unknown): Promise<{ success: boolean; error?: string }> =>
      ipcRenderer.invoke('lsp:notify', method, params),

    isRunning: (): Promise<{ running: boolean }> =>
      ipcRenderer.invoke('lsp:isRunning'),

    // Server → Client notification listeners
    onDiagnostics: (callback: (params: LSPDiagnosticsParams) => void): (() => void) => {
      const handler = (_: unknown, params: LSPDiagnosticsParams) => callback(params)
      ipcRenderer.on('lsp:diagnostics', handler)
      return () => ipcRenderer.removeListener('lsp:diagnostics', handler)
    },

    onLog: (callback: (params: { type: string; message: string }) => void): (() => void) => {
      const handler = (_: unknown, params: { type: string; message: string }) => callback(params)
      ipcRenderer.on('lsp:log', handler)
      return () => ipcRenderer.removeListener('lsp:log', handler)
    },

    onError: (callback: (params: { message: string }) => void): (() => void) => {
      const handler = (_: unknown, params: { message: string }) => callback(params)
      ipcRenderer.on('lsp:error', handler)
      return () => ipcRenderer.removeListener('lsp:error', handler)
    },

    onClose: (callback: (params: { code: number }) => void): (() => void) => {
      const handler = (_: unknown, params: { code: number }) => callback(params)
      ipcRenderer.on('lsp:close', handler)
      return () => ipcRenderer.removeListener('lsp:close', handler)
    },

    removeListeners: (): void => {
      ipcRenderer.removeAllListeners('lsp:diagnostics')
      ipcRenderer.removeAllListeners('lsp:log')
      ipcRenderer.removeAllListeners('lsp:error')
      ipcRenderer.removeAllListeners('lsp:close')
    }
  }
})

// TypeScript declaration for window.qalam
declare global {
  interface Window {
    qalam: {
      file: {
        open: () => Promise<FileData | FileError | null>
        read: (path: string) => Promise<{ content: string } | null>
        save: (path: string, content: string) => Promise<SaveResult>
        saveAs: (content: string) => Promise<SaveResult | null>
      }
      folder: {
        open: () => Promise<{ path: string; name: string } | null>
        read: (path: string) => Promise<Array<{ name: string; path: string; type: 'file' | 'directory' }>>
        createFile: (path: string) => Promise<{ success?: boolean; error?: string }>
        createFolder: (path: string) => Promise<{ success?: boolean; error?: string }>
        rename: (oldPath: string, newPath: string) => Promise<{ success?: boolean; error?: string }>
        delete: (path: string) => Promise<{ success?: boolean; error?: string }>
        reveal: (path: string) => Promise<{ success: boolean }>
      }
      project: {
        exists: (folderPath: string) => Promise<{ exists: boolean }>
        read: (folderPath: string) => Promise<{ success: boolean; manifest?: unknown; error?: string }>
        write: (folderPath: string, manifest: unknown) => Promise<{ success: boolean; error?: string }>
        init: (folderPath: string, projectName: string) => Promise<{ success: boolean; error?: string }>
      }
      compiler: {
        compile: (filePath: string) => Promise<CompilerResult>
        run: (filePath: string) => Promise<CompilerResult>
        onStdout: (callback: (output: string) => void) => void
        onStderr: (callback: (error: string) => void) => void
        removeListeners: () => void
      }
      menu: {
        onOpen: (callback: () => void) => () => void
        onSave: (callback: () => void) => () => void
        onSaveAs: (callback: () => void) => () => void
        onCompile: (callback: () => void) => () => void
        onRun: (callback: () => void) => () => void
      }
      lsp: {
        start: (workspacePath: string) => Promise<LSPStartResult>
        stop: () => Promise<{ success: boolean; error?: string }>
        request: (method: string, params: unknown) => Promise<LSPRequestResult>
        notify: (method: string, params: unknown) => Promise<{ success: boolean; error?: string }>
        isRunning: () => Promise<{ running: boolean }>
        onDiagnostics: (callback: (params: LSPDiagnosticsParams) => void) => () => void
        onLog: (callback: (params: { type: string; message: string }) => void) => () => void
        onError: (callback: (params: { message: string }) => void) => () => void
        onClose: (callback: (params: { code: number }) => void) => () => void
        removeListeners: () => void
      }
    }
  }
}

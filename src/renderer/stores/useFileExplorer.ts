import { create } from 'zustand'
import { persist } from 'zustand/middleware'

export interface FileNode {
  name: string
  path: string
  type: 'file' | 'directory'
  children?: FileNode[]
  isExpanded?: boolean
}

interface FileExplorerState {
  rootPath: string | null
  rootName: string | null
  tree: FileNode[]
  expandedPaths: Set<string>
  isLoading: boolean

  // Actions
  setRoot: (path: string, name: string) => void
  setTree: (tree: FileNode[]) => void
  toggleExpanded: (path: string) => void
  setExpanded: (path: string, expanded: boolean) => void
  updateChildren: (path: string, children: FileNode[]) => void
  closeFolder: () => void
  setLoading: (loading: boolean) => void
}

export const useFileExplorer = create<FileExplorerState>()(
  persist(
    (set, get) => ({
      rootPath: null,
      rootName: null,
      tree: [],
      expandedPaths: new Set<string>(),
      isLoading: false,

      setRoot: (path: string, name: string) => {
        set({
          rootPath: path,
          rootName: name,
          tree: [],
          expandedPaths: new Set<string>()
        })
      },

      setTree: (tree: FileNode[]) => {
        set({ tree })
      },

      toggleExpanded: (path: string) => {
        const { expandedPaths } = get()
        const newExpanded = new Set(expandedPaths)
        if (newExpanded.has(path)) {
          newExpanded.delete(path)
        } else {
          newExpanded.add(path)
        }
        set({ expandedPaths: newExpanded })
      },

      setExpanded: (path: string, expanded: boolean) => {
        const { expandedPaths } = get()
        const newExpanded = new Set(expandedPaths)
        if (expanded) {
          newExpanded.add(path)
        } else {
          newExpanded.delete(path)
        }
        set({ expandedPaths: newExpanded })
      },

      updateChildren: (path: string, children: FileNode[]) => {
        const { tree } = get()

        const updateNode = (nodes: FileNode[]): FileNode[] => {
          return nodes.map(node => {
            if (node.path === path) {
              return { ...node, children }
            }
            if (node.children) {
              return { ...node, children: updateNode(node.children) }
            }
            return node
          })
        }

        set({ tree: updateNode(tree) })
      },

      closeFolder: () => {
        set({
          rootPath: null,
          rootName: null,
          tree: [],
          expandedPaths: new Set<string>()
        })
      },

      setLoading: (loading: boolean) => {
        set({ isLoading: loading })
      }
    }),
    {
      name: 'qalam-file-explorer',
      partialize: (state) => ({
        rootPath: state.rootPath,
        rootName: state.rootName,
        // Convert Set to Array for serialization
        expandedPaths: Array.from(state.expandedPaths)
      }),
      merge: (persisted: any, current) => ({
        ...current,
        ...persisted,
        // Convert Array back to Set
        expandedPaths: new Set(persisted?.expandedPaths || [])
      })
    }
  )
)

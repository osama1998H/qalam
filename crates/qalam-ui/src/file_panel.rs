//! لوحة الملفات - File panel component

use crate::theme::Theme;
use iced::widget::{button, column, container, scrollable, text, Column};
use iced::{Element, Length};
use std::path::{Path, PathBuf};

/// عنصر الشجرة - Tree item
#[derive(Debug, Clone)]
pub struct FileItem {
    /// المسار - Path
    pub path: PathBuf,
    /// الاسم - Display name
    pub name: String,
    /// هل هو مجلد - Is directory
    pub is_dir: bool,
    /// مفتوح - Is expanded (for directories)
    pub expanded: bool,
    /// مستوى التداخل - Nesting level
    pub level: usize,
}

/// لوحة الملفات - File panel state
pub struct FilePanel {
    /// العناصر - Items
    items: Vec<FileItem>,
    /// المسار الجذر - Root path
    root: Option<PathBuf>,
    /// العنصر المحدد - Selected item
    selected: Option<usize>,
}

/// رسائل لوحة الملفات - File panel messages
#[derive(Debug, Clone)]
pub enum FilePanelMessage {
    /// تحديد/فتح عنصر - Select/open item
    ItemClicked(usize),
}

impl FilePanel {
    /// إنشاء لوحة جديدة - Create new panel
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            root: None,
            selected: None,
        }
    }

    /// تعيين المجلد الجذر - Set root directory
    pub fn set_root(&mut self, path: PathBuf) {
        self.root = Some(path.clone());
        self.items.clear();
        self.load_directory(&path, 0);
    }

    /// تحميل محتويات المجلد - Load directory contents
    fn load_directory(&mut self, path: &Path, level: usize) {
        if let Ok(entries) = std::fs::read_dir(path) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();

            for entry in entries.flatten() {
                let path = entry.path();
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                // تجاهل الملفات المخفية
                if name.starts_with('.') {
                    continue;
                }

                let item = FileItem {
                    path: path.clone(),
                    name,
                    is_dir: path.is_dir(),
                    expanded: false,
                    level,
                };

                if item.is_dir {
                    dirs.push(item);
                } else {
                    files.push(item);
                }
            }

            // ترتيب: المجلدات أولاً ثم الملفات
            dirs.sort_by(|a, b| a.name.cmp(&b.name));
            files.sort_by(|a, b| a.name.cmp(&b.name));

            self.items.extend(dirs);
            self.items.extend(files);
        }
    }

    /// معالجة الرسالة - Handle message
    pub fn update(&mut self, message: FilePanelMessage) -> Option<PathBuf> {
        match message {
            FilePanelMessage::ItemClicked(index) => {
                if let Some(item) = self.items.get_mut(index) {
                    if item.is_dir {
                        // Toggle directory expansion
                        item.expanded = !item.expanded;
                        if item.expanded {
                            let path = item.path.clone();
                            let level = item.level + 1;
                            let insert_at = index + 1;

                            let mut new_items = Vec::new();
                            if let Ok(entries) = std::fs::read_dir(&path) {
                                for entry in entries.flatten() {
                                    let entry_path = entry.path();
                                    let name = entry_path
                                        .file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("")
                                        .to_string();

                                    if name.starts_with('.') {
                                        continue;
                                    }

                                    new_items.push(FileItem {
                                        path: entry_path.clone(),
                                        name,
                                        is_dir: entry_path.is_dir(),
                                        expanded: false,
                                        level,
                                    });
                                }
                            }

                            new_items.sort_by(|a, b| {
                                match (a.is_dir, b.is_dir) {
                                    (true, false) => std::cmp::Ordering::Less,
                                    (false, true) => std::cmp::Ordering::Greater,
                                    _ => a.name.cmp(&b.name),
                                }
                            });

                            for (i, new_item) in new_items.into_iter().enumerate() {
                                self.items.insert(insert_at + i, new_item);
                            }
                        } else {
                            let level = item.level;
                            let mut remove_count = 0;
                            for i in (index + 1)..self.items.len() {
                                if self.items[i].level > level {
                                    remove_count += 1;
                                } else {
                                    break;
                                }
                            }
                            self.items.drain((index + 1)..(index + 1 + remove_count));
                        }
                        None
                    } else {
                        // Open file
                        self.selected = Some(index);
                        Some(item.path.clone())
                    }
                } else {
                    None
                }
            }
        }
    }

    /// عرض اللوحة - Render panel
    pub fn view(&self, theme: &Theme) -> Element<FilePanelMessage> {
        // Clone colors needed
        let panel_bg = theme.panel_background;
        let border_color = theme.border;
        let selection_color = theme.selection;

        let mut content: Column<FilePanelMessage> = Column::new().spacing(2);

        for (index, item) in self.items.iter().enumerate() {
            let indent = "  ".repeat(item.level);
            let icon = if item.is_dir {
                if item.expanded { "📂" } else { "📁" }
            } else {
                match item.path.extension().and_then(|e| e.to_str()) {
                    Some("trq") | Some("ترق") => "📝",
                    Some("md") => "📄",
                    Some("toml") | Some("json") => "⚙️",
                    _ => "📄",
                }
            };

            let label = format!("{}{} {}", indent, icon, item.name);
            let is_selected = self.selected == Some(index);
            let bg_color = if is_selected { selection_color } else { panel_bg };

            let item_button = button(
                text(label).size(14)
            )
            .width(Length::Fill)
            .padding(4)
            .style(move |_theme, _status| {
                button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: iced::Color::WHITE,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                }
            })
            .on_press(FilePanelMessage::ItemClicked(index));

            content = content.push(item_button);
        }

        let panel = container(
            scrollable(content).height(Length::Fill)
        )
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .padding(8)
        .style(move |_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Background::Color(panel_bg)),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        });

        panel.into()
    }
}

impl Default for FilePanel {
    fn default() -> Self {
        Self::new()
    }
}

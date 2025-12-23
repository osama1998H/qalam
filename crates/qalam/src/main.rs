//! # قلم - Qalam
//! محرر أكواد عربي للغة ترقيم
//! Arabic RTL Code Editor for Tarqeem

fn main() -> iced::Result {
    // تهيئة السجل
    env_logger::init();

    log::info!("بدء تشغيل قلم...");

    // تشغيل التطبيق
    qalam_ui::run()
}

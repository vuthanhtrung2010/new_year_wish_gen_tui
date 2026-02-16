use colored::*;
use console::Term;
use inquire::{
    MultiSelect, Select, Text,
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
};
use std::{thread, time::Duration};
use unicode_width::UnicodeWidthStr;

#[cfg(target_os = "windows")]
use arboard::Clipboard;

#[cfg(target_os = "linux")]
use std::env;
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::process::{Command, Stdio};

fn main() {
    setup_ui();

    let term = Term::stdout();
    term.clear_screen().unwrap();

    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    println!(
        "\n{}",
        "◇  LUNAR NEW YEAR WISH GENERATOR  ◇"
            .bold()
            .truecolor(255, 62, 0)
    );

    let options = vec!["Giáo viên (Thầy/Cô)", "Bạn bè / Homies", "Thoát"];

    let choice = Select::new("Bạn muốn chúc ai?", options)
        .with_help_message("Enter để xác nhận")
        .prompt();

    let content = match choice {
        Ok(c) => {
            if c.contains("Giáo viên") {
                handle_teacher()
            } else if c.contains("Bạn bè") {
                handle_friend()
            } else {
                return;
            }
        }
        Err(_) => return,
    };

    if let Some(wish) = content {
        print_box_result(&wish);
    }
}

fn handle_teacher() -> Option<String> {
    let genders = vec!["Nam (thầy)", "Nữ (cô)"];
    let gender_select = Select::new("Giới tính giáo viên?", genders)
        .prompt()
        .unwrap();
    let title = if gender_select.contains("Nam") {
        "thầy"
    } else {
        "cô"
    };

    let name = Text::new("Tên thầy/cô là gì?")
        .with_placeholder("ví dụ: Phương")
        .prompt()
        .unwrap();

    spinner("Generating...");

    Some(format!(
        "Nhân dịp năm mới, con chúc {} {} và gia đình luôn vạn sự như ý, tỷ sự như mơ, triệu triệu bất ngờ, và luôn gặt hái thêm nhiều thành công rực rỡ trong sự nghiệp trồng người ạ! :D",
        title,
        name.trim()
    ))
}

fn handle_friend() -> Option<String> {
    let name = Text::new("Tên đứa bạn?")
        .with_placeholder("ví dụ: Vy, Nam")
        .prompt()
        .unwrap();

    let gender = Select::new("Giới tính?", vec!["Nam", "Nữ"])
        .prompt()
        .unwrap();

    let is_dt_tin = Select::new("Có phải đội Dự tuyển Tin không?", vec!["Có", "Không"])
        .prompt()
        .unwrap();

    let wish_types = vec![
        "Học tập / Điểm số",
        "Nhan sắc",
        "Tình duyên",
        "Tiền bạc",
        "Code / IT",
    ];

    let selections = MultiSelect::new("Chọn lời chúc (Space để chọn):", wish_types)
        .prompt()
        .unwrap();

    if selections.is_empty() {
        return None;
    }

    spinner("Generating...");

    let mut parts = Vec::new();

    for selection in selections {
        match selection {
            "Học tập / Điểm số" => {
                if is_dt_tin == "Có" {
                    parts.push("điểm LAH thật cao để được miễn thi".to_string());
                } else {
                    parts
                        .push("điểm công bằng và cuối kì cao chót vót để săn học bổng 👀".to_string());
                }
            }
            "Nhan sắc" => {
                if gender == "Nam" {
                    parts.push("ngày càng đẹp trai, phong độ ngời ngời".to_string());
                } else {
                    parts.push("ngày càng xinh đẹp, trẻ trung rạng rỡ".to_string());
                }
            }
            "Tình duyên" => {
                parts.push("đào hoa nở rộ, sớm có người yêu xịn sò".to_string());
            }
            "Tiền bạc" => {
                parts.push("tiền lì xì đếm mỏi tay, ví lúc nào cũng dày".to_string());
            }
            "Code / IT" => {
                parts.push("code một phát ăn ngay, bug tự fix, AK đề".to_string());
            }
            _ => {}
        }
    }

    let joined_wishes = match parts.len() {
        0 => return None,
        1 => parts[0].clone(),
        _ => {
            let last = parts.pop().unwrap();
            format!("{} và {}", parts.join(", "), last)
        }
    };

    Some(format!(
        "Nhân dịp năm mới, chúc {} {} nhé! ✨🧨",
        name.trim(),
        joined_wishes
    ))
}

fn universal_copy(text: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(mut clipboard) = Clipboard::new() {
            if clipboard.set_text(text).is_ok() {
                return Ok("Copied using Windows API".to_string());
            }
        }
        return Err("Clipboard error".to_string());
    }

    #[cfg(target_os = "linux")]
    {
        let is_wayland = env::var("WAYLAND_DISPLAY").is_ok();
        let tools = if is_wayland {
            vec![("wl-copy", vec![])]
        } else {
            vec![
                ("xclip", vec!["-selection", "clipboard"]),
                ("xsel", vec!["--clipboard", "--input"]),
            ]
        };

        for (tool, args) in tools {
            // Check if tool exists
            if Command::new("which").arg(tool).output().is_ok() {
                let mut child = Command::new(tool)
                    .args(args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|_| "Spawn error")?;

                if let Some(mut stdin) = child.stdin.take() {
                    stdin
                        .write_all(text.as_bytes())
                        .map_err(|_| "Write error")?;
                }

                let _ = child.wait();
                return Ok(format!("Copied using {}", tool));
            }
        }
        return Err("No clipboard tool found (install wl-clipboard or xclip)".to_string());
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err("OS not supported".to_string())
    }
}

fn setup_ui() {
    let mut config = RenderConfig::default();
    let svelte_orange = Color::AnsiValue(208);
    config.prompt_prefix = Styled::new("?").with_fg(svelte_orange);
    config.answered_prompt_prefix = Styled::new("✔").with_fg(Color::LightGreen);
    config.selected_option = Some(
        StyleSheet::new()
            .with_fg(svelte_orange)
            .with_attr(Attributes::BOLD),
    );
    config.selected_checkbox = Styled::new("◉").with_fg(Color::LightGreen);
    config.unselected_checkbox = Styled::new("◯").with_fg(Color::DarkGrey);

    inquire::set_global_render_config(config);
}

fn spinner(msg: &str) {
    let term = Term::stdout();
    let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    term.hide_cursor().unwrap();
    for _ in 0..3 {
        for frame in &frames {
            term.write_line(&format!("{}  {}", frame.truecolor(255, 62, 0), msg))
                .unwrap();
            term.move_cursor_up(1).unwrap();
            thread::sleep(Duration::from_millis(30));
            term.clear_line().unwrap();
        }
    }
    term.show_cursor().unwrap();
}

fn print_box_result(content: &str) {
    let width = 60;
    let content_width = width - 4;

    println!("\n{}", format!("┌{}┐", "─".repeat(width - 2)).dimmed());
    println!("{}  {}", "✨".yellow(), "RESULT:".bold().white());
    println!("{}", format!("├{}┤", "─".repeat(width - 2)).dimmed());

    let wrapped = textwrap::wrap(content, content_width);

    for line in wrapped {
        let padding = content_width - UnicodeWidthStr::width(line.as_ref());
        println!("│  {}{}  │", line.bright_white(), " ".repeat(padding));
    }

    println!("{}", format!("└{}┘", "─".repeat(width - 2)).dimmed());

    match universal_copy(content) {
        Ok(msg) => println!("   ✅ {}   \n", msg.italic().green()),
        Err(e) => println!("   ❌ {} ({})   \n", "Copy failed".red(), e.dimmed()),
    }

    #[cfg(windows)]
    let _ = Text::new("Press Enter to exit...").prompt();
}

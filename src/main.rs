use clap::{Parser, Subcommand};
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "kubecfg")]
#[clap(about = "Kubernetes 配置管理工具", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 切换当前使用的配置
    Use { config: String },
    /// 保存配置
    Add { config: String },
    /// 保存配置
    Save { config: String },
    /// 删除配置
    Remove { config: String },
    /// 列出所有配置
    List,
}

// 默认配置路径
fn default_config() -> PathBuf {
    home_dir()
        .expect("无法找到用户目录")
        .join(".kube")
        .join("config")
}

/// 存储配置的目录
fn config_dir() -> PathBuf {
    home_dir()
        .expect("无法找到用户目录")
        .join(".kube")
        .join("config.d")
}

/// 获取指定配置文件路径
fn config_file(config: &str) -> PathBuf {
    config_dir().join(config)
}

fn use_config(config: &str) {
    let src_path = config_file(config);
    if !src_path.exists() {
        eprintln!("配置 '{}' 不存在", config);
        return;
    }

    let dst_path = default_config();

    // 确保目标目录存在
    if let Some(parent_dir) = dst_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)
                .expect(format!("无法创建目录: {}", parent_dir.display()).as_str());
        }
    }

    // 执行复制操作
    match fs::copy(&src_path, &dst_path) {
        Ok(_) => println!("已切换到配置 '{}'", config),
        Err(e) => eprintln!("无法复制配置文件: {}", e),
    }
}

fn add_config(config: &str) {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).expect(format!("无法创建目录: {}", dir.display()).as_str());
    }

    let dst_path = config_file(config);
    let src_path = default_config();

    if !src_path.exists() {
        eprintln!("默认配置文件不存在：{}", src_path.display());
        return;
    }

    // 执行复制操作（无论文件是否存在）
    match fs::copy(&src_path, &dst_path) {
        Ok(_) => println!("已保存配置 '{}'", config),
        Err(e) => eprintln!("无法保存配置文件: {}", e),
    }
}

fn remove_config(config: &str) {
    let path = config_file(config);
    if !path.exists() {
        eprintln!("配置 '{}' 不存在", config);
        return;
    }

    fs::remove_file(path).expect("无法删除配置");
    println!("已删除配置 '{}'", config);
}

fn list_configs() {
    let dir = config_dir();
    if !dir.exists() {
        return;
    }

    let configs =
        fs::read_dir(&dir).expect(format!("无法读取配置目录: {}", &dir.display()).as_str());
    for e in configs {
        let config = e.expect("无法读取配置");
        let path = config.path();
        if path.is_file() {
            let filename = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect(format!("无效的配置: {}", path.display()).as_str());
            println!("{}", filename);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Use { config } => use_config(config),
        Commands::Add { config } => add_config(config),
        Commands::Save { config } => add_config(config),
        Commands::Remove { config } => remove_config(config),
        Commands::List => list_configs(),
    }
}

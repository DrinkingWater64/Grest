use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};
use walkdir::WalkDir;
use clap::{Parser, ArgAction};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Root directory to analyze
    #[arg(default_value = ".")]
    root_path: PathBuf,

    /// Output file path
    #[arg(short, long, default_value = "code_output.txt")]
    output: PathBuf,

    /// Directories to ignore during scanning
    #[arg(short, long, default_value_t = String::from(".git,node_modules,target,.idea,venv,bin,obj,Debug,Release"))]
    ignored_dirs: String,

    /// File extensions to include
    #[arg(short, long, default_value_t = String::from("rs,js,py,cpp,c,java,go,ts,cs,csproj,sln,cshtml,razor,json,xml,config,yml,yaml"))]
    extensions: String,

    /// Verbose output
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

struct Config {
    root_path: PathBuf,
    output_file: PathBuf,
    ignored_dirs: Vec<String>,
    allowed_extensions: Vec<String>,
    verbose: bool,
}

impl Config {
    fn new(cli: Cli) -> Self {
        Config {
            root_path: cli.root_path,
            output_file: cli.output,
            ignored_dirs: cli.ignored_dirs.split(',').map(|s| s.to_string()).collect(),
            allowed_extensions: cli.extensions.split(',').map(|s| s.to_string()).collect(),
            verbose: cli.verbose,
        }
    }
}

fn count_total_files(config: &Config) -> io::Result<(usize, usize)> {
    let mut total_files = 0;
    let mut code_files = 0;

    for entry in WalkDir::new(&config.root_path)
        .into_iter()
        .filter_entry(|e| !is_ignored(e, &config.ignored_dirs))
    {
        let entry = entry?;
        if entry.file_type().is_file() {
            total_files += 1;
            
            if let Some(extension) = entry.path().extension() {
                if config.allowed_extensions.contains(&extension.to_string_lossy().to_string()) {
                    code_files += 1;
                }
            }
        }
    }

    Ok((total_files, code_files))
}

fn process_directory(config: &Config) -> io::Result<()> {
    // Count total files first
    let (total_files, code_files) = count_total_files(config)?;

    // Create progress bar
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Create output file
    let mut output = File::create(&config.output_file)?;
    
    if config.verbose {
        println!("Total files: {}, Code files: {}", total_files, code_files);
    }

    writeln!(output, "Directory Tree and Code Contents\n")?;
    writeln!(output, "Root Directory: {}\n", config.root_path.display())?;
    writeln!(output, "Total Files: {}\n", total_files)?;
    writeln!(output, "Code Files: {}\n", code_files)?;

    // Generate directory tree
    for entry in WalkDir::new(&config.root_path)
        .into_iter()
        .filter_entry(|e| !is_ignored(e, &config.ignored_dirs))
    {
        let entry = entry?;
        let path = entry.path();
        
        let depth = entry.depth();
        let prefix = "    ".repeat(depth);
        
        let name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy();
            
        writeln!(output, "{}{}{}", prefix, "├── ", name)?;
    }

    writeln!(output, "\nCode Contents:\n")?;

    // Process code files
    for entry in WalkDir::new(&config.root_path)
        .into_iter()
        .filter_entry(|e| !is_ignored(e, &config.ignored_dirs))
    {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        pb.inc(1);

        let path = entry.path();
        
        if let Some(extension) = path.extension() {
            if config.allowed_extensions.contains(&extension.to_string_lossy().to_string()) {
                writeln!(output, "\n=== File: {} ===\n", path.display())?;
                
                match fs::read_to_string(path) {
                    Ok(contents) => {
                        writeln!(output, "{}", contents)?;
                    }
                    Err(e) => {
                        writeln!(output, "Error reading file: {}", e)?;
                    }
                }
            }
        }
    }

    pb.finish_with_message("Scan complete");
    
    println!("Successfully generated code output at: {}", config.output_file.display());
    Ok(())
}

fn is_ignored(entry: &walkdir::DirEntry, ignored_dirs: &[String]) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| ignored_dirs.iter().any(|ignored| s == ignored))
        .unwrap_or(false)
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    let config = Config::new(cli);
    
    if config.verbose {
        println!("Analyzing directory: {}", config.root_path.display());
        println!("Output will be written to: {}", config.output_file.display());
        println!("Ignored directories: {:?}", config.ignored_dirs);
        println!("Allowed extensions: {:?}", config.allowed_extensions);
    }
    
    process_directory(&config)
}
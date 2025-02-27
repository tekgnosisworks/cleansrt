use std::{ffi::OsString, path::PathBuf, fs};
use srtlib::{Subtitle, Subtitles};
use clap::{value_parser, Arg, ArgGroup, Command};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use log::{error, info};
use env_logger::{Builder, Target};

fn main() {
    if let Err(e) =setup_logger(){
        eprintln!("Failed to setup logger: {}", e);
    }
    let matches = Command::new("cleansrt")
        .version("1.0")
        .author("William Moore bmoore@tekgnosis.works")
        .about("Removes unwanted text from srt files")
        .arg(
            Arg::new("input")
                .short('f')
                .long("file")
                .value_name("FILE_OR_DIR")
                .value_parser(value_parser!(PathBuf))
                .required(true)
                .help("Sets the srt file, or folder to scan"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .value_parser(value_parser!(PathBuf))
                .help("Optional, output file. For directories, output will be a directory. Otherwise files will be .EDITED.srt"),
        )
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .action(clap::ArgAction::SetTrue)
                .help("Delete the OLD files after processing"),
        )
        .arg(
            Arg::new("text")
                .short('t')
                .long("text")
                .value_name("TEXT")
                .help("Enter text string to filter. Use \\n for newlines.")
                .conflicts_with("text_file"),
        )
        .arg(
            Arg::new("text_file")
                .short('T')
                .long("text-file")
                .value_name("TEXT_FILE")
                .value_parser(value_parser!(PathBuf))
                .help("File of text to scan for. Each instance on a single line. Use \\n for newline in each instance")
                .conflicts_with("text"),
        )
        .group(
            ArgGroup::new("text_source")
                .args(&["text", "text_file"])
                .required(true),
        )
        .get_matches();

    let input_path: &PathBuf = matches.get_one("input").unwrap();
    info!("Start at {}", input_path.to_str().unwrap());
    let mut texts: Vec<String> = Vec::new();
    let delete_old = matches.get_flag("delete");
    if matches.contains_id("text") {
        let holder: &String = matches.get_one("text").unwrap();
        texts.push(holder.replace("\\n", "\n").to_owned());
    } else {
        let holder = matches.get_one("text_file").unwrap();
        texts = read_to_vec(holder);
    };
    info!("   Remove from srt: {:?}",texts);
    if input_path.is_file() {
        let output_file = determine_output_file(input_path, matches.get_one("output"));
        process_srt_file(input_path, &output_file, &texts);
    } else if input_path.is_dir() {
        let base_output_dir = input_path.clone();
        process_directory(input_path, &base_output_dir, &texts, delete_old);
    } else {
        error!("Error: Input path does not exist or is not accessible");
        std::process::exit(1);
    }
}


fn determine_output_file(input_file: &PathBuf, output_option: Option<&PathBuf>) -> PathBuf {
    if let Some(output_file) = output_option {
        output_file.clone()
    } else {
        let mut filename: OsString = input_file.file_stem().unwrap().to_os_string();
        filename.push(".EDITED.");
        filename.push(input_file.extension().unwrap().to_str().unwrap());
        let mut output_path = input_file.to_path_buf();
        output_path.set_file_name(filename);
        output_path
    }
}

fn process_srt_file(input_file: &PathBuf, output_file: &PathBuf, texts: &[String]) {
    match Subtitles::parse_from_file(input_file, None) {
        Ok(subs) => {
            let mut working = Subtitles::new();
            let mut offset_count: usize = 0;
            
            for s in subs {
                let mut check = false;
                for t in texts {
                    if s.text == t.to_string() {
                        check = true;
                        offset_count = offset_count + 1;
                    }
                }
                if !check {
                    let mut edit_sub: Subtitle = s;
                    if edit_sub.num > 1 {
                        edit_sub.num = edit_sub.num - offset_count;
                    }
                    working.push(edit_sub);
                }
            }

            if let Err(e) = working.write_to_file(output_file, None) {
                error!("Error writing to file {:?}: {}", output_file, e);
            } else {
                info!("Processed: {:?} -> {:?}", input_file, output_file);
            }
        },
        Err(e) => {
            error!("Error parsing SRT file {:?}: {}", input_file, e);
        }
    }
}

fn process_directory(input_dir: &PathBuf, base_output_dir: &PathBuf, texts: &[String], delete: bool) {
    let mut srt_files = Vec::new();
    find_srt_files(input_dir, &mut srt_files);
    info!("Found {} .srt files to process", srt_files.len());
    //todo add in percentage calculator
    for file_path in srt_files {
        let rel_path = file_path.strip_prefix(input_dir).unwrap_or(&file_path);
        let mut output_path = base_output_dir.join(rel_path);
        let mut filename: OsString = output_path.file_stem().unwrap().to_os_string();
        filename.push(".OLD.");
        filename.push(output_path.extension().unwrap().to_str().unwrap());
        output_path.set_file_name(filename);

        let _ = fs::rename(&file_path, &output_path);
        process_srt_file(&output_path, &file_path, texts);
        if delete {
            let _ = fs::remove_file(&output_path);
        }
    }
}

fn find_srt_files(dir: &PathBuf, srt_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                find_srt_files(&path, srt_files);
            } else if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_string_lossy().to_lowercase() == "srt" {
                        srt_files.push(path);
                    }
                }
            }
        }
    }
}

fn read_to_vec(file: &PathBuf) -> Vec<String> {
    match File::open(file) {
        Ok(file) => {
            let reader = BufReader::new(&file);
            match reader.lines().collect::<Result<Vec<String>, _>>() {
                Ok(lines) => {
                    let mut return_vec: Vec<String> = Vec::new();
                    for line in lines {
                        return_vec.push(line.replace("\\n", "\n"));
                    }
                    return_vec
                },
                Err(e) => {
                    error!("Error reading lines from file {:?}: {}", file, e);
                    Vec::new()
                }
            }
        },
        Err(e) => {
            error!("Error opening file {:?}: {}", file, e);
            Vec::new()
        }
    }
}

fn setup_logger() -> Result<(), io::Error> {
    let current_exe_dir = std::env::current_exe()?
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not get executable directory"))?
        .to_path_buf();
    
    let log_file_path = current_exe_dir.join("cleansrt.log");
    
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)?;

    let multi_writer = MultiWriter::new(vec![
        Box::new(file),
        Box::new(io::stderr()),
    ]);

    let mut builder = Builder::from_default_env();

    builder.filter_level(log::LevelFilter::Info);
    
    builder
        .target(Target::Pipe(Box::new(multi_writer)))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    Ok(())
}

struct MultiWriter {
    writers: Vec<Box<dyn Write + Send + 'static>>,
}

impl MultiWriter {
    fn new(writers: Vec<Box<dyn Write + Send + 'static>>) -> Self {
        MultiWriter { writers }
    }
}

impl Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for writer in &mut self.writers {
            writer.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        for writer in &mut self.writers {
            writer.flush()?;
        }
        Ok(())
    }
}
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version)]
pub struct CommandLineArguments {
    /// ファイルパス
    #[arg(index = 1, value_name = "SisPassMgrCsv.csv")]
    pub file_path: String,
}

/// コマンドライン引数をパースする。
/// 別で関数を作成した方が記述が簡潔になるため、このような形にしている。
pub fn parse_command_line_arguments() -> CommandLineArguments {
    CommandLineArguments::parse()
}

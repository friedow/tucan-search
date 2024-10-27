use rust_search::FilterExt;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let git_directory_paths: Vec<String> = rust_search::SearchBuilder::default()
        .location("~/")
        .search_input("\\.git")
        .limit(100)
        .strict()
        .hidden()
        .custom_filter(|dir| {
            let Some(name) = dir.file_name().to_str() else {
                return false;
            };
            if name == ".git" {
                return true;
            }
            !name.starts_with('.')
        })
        .build()
        .collect();

    let git_repository_paths: Vec<&str> = git_directory_paths
        .iter()
        .filter_map(|git_directory_path| {
            let git_repository_path_option = git_directory_path.strip_suffix("/.git");
            if git_repository_path_option.is_none() {
                log::warn!(
                    "Unable to strip '/.git' suffix from path '{}'",
                    git_directory_path
                );
            }
            git_repository_path_option
        })
        .collect();

    write_index_file(git_repository_paths);
}

fn write_index_file(git_repository_paths: Vec<&str>) {
    let cache_directory_result = settings::centerpiece_cache_directory();
    if let Err(error) = cache_directory_result {
        log::error!(
        error = log::error!("{:?}", error);
        "Could not determine cache directory.",
        );
        panic!();
    }
    let centerpice_cache_directory = cache_directory_result.unwrap();

    if let Err(error) = std::fs::create_dir_all(&centerpice_cache_directory) {
        log::error!(
        error = log::error!("{:?}", error);
        "Error while creating cache directory",
        );
        panic!();
    }

    let index_file_path =
        std::path::Path::new(&centerpice_cache_directory).join("git-repositories-index.json");

    let index_file_result = std::fs::File::create(index_file_path);
    if let Err(error) = index_file_result {
        log::error!(
        error = log::error!("{:?}", error);
        "Error while creating index file",
        );
        panic!();
    }
    let index_file = index_file_result.unwrap();

    let mut writer = std::io::BufWriter::new(index_file);
    if let Err(error) = serde_json::to_writer(&mut writer, &git_repository_paths) {
        log::error!(
        error = log::error!("{:?}", error);
        "Error while writing index file",
        );
        panic!();
    }
}

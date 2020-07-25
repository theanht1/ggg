use std::{fs};


#[derive(Debug)]
pub struct Template {
    pub name: String,
    pub path: String,
}

impl Template {
    fn from_dir_entry(dir_entry: fs::DirEntry) -> Template {
        let name = dir_entry.file_name().into_string().unwrap()
             .split(".").collect::<Vec<&str>>()[0]
             .into();
        let path = dir_entry.path().into_os_string().into_string().unwrap();
        Template {
            name: name,
            path: path,
        }
    }
}

pub fn list_ignore_templates(path: &str) -> Vec<Template> {
    let mut entries = fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|f| f.path().to_str().and_then(|file| if file.ends_with(".gitignore") { Some(f) } else { None }))
        .map(|f| Template::from_dir_entry(f))
        .collect::<Vec<Template>>();

    entries.sort_by(|a, b| a.name.cmp(&b.name));
    entries
}

pub fn list_templates() -> Vec<Template> {
    let templates = list_ignore_templates("./gitignore");
    templates
}

pub fn find_template(template_name: &str) -> Option<Template> {
    let templates = list_templates();
    for template in templates {
        if template.name == template_name {
            return Some(template)
        }
    }
    None
}

pub fn create_file(template: Template, outfile_path: &str) {
    fs::copy(template.path, outfile_path).unwrap();
}

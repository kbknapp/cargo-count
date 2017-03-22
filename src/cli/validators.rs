use fmt::Format;

pub fn supported_lang(s: String) -> Result<(), String> {
    if Language::from_ext(&*s).is_none() {
        return Err(format!("unsupported source code extension '{}'", Format::Warning(s)));
    }
    Ok(())
}

pub fn single_char(s: String) -> Result<(), String> {
    if s.len() != 1 {
        return Err(format!("the --separator argument option only accepts a single character but \
                            found '{}'",
                           Format::Warning(s)));
    }
    Ok(())
}
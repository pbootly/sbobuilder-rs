use core::fmt;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct SboInfoError;

impl fmt::Display for SboInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Package info has errors, unable to proceed")
    }
}

#[derive(Debug, Clone)]
pub struct RequiresError;

impl fmt::Display for RequiresError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%README% found in requires, unable to proceed")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SboInfo {
    pub program_name: String,
    version: String,
    homepage: String,
    download: Vec<String>,
    md5sum: Vec<String>,
    download_x86_64: Vec<String>,
    md5sum_x86_64: Vec<String>,
    pub requires: Vec<String>,
    maintainer: String,
    email: String,
}

impl SboInfo {
    pub fn new() -> Self {
        SboInfo::default()
    }

    pub fn from_str(&mut self, contents: &str) -> Result<(), SboInfoError> {
        let re = Regex::new(r#"(\w+)=\"([^\"]*)\""#).unwrap();
        for (_, [key, value]) in re.captures_iter(contents).map(|c| c.extract()) {
            match key {
                "PRGNAM" => {
                    self.program_name(value);
                }
                "VERSION" => {
                    self.version(value);
                }
                "HOMEPAGE" => {
                    self.homepage(value);
                }
                "DOWNLOAD" => {
                    self.download(value);
                }
                "MD5SUM" => {
                    self.md5sum(value);
                }
                "DOWNLOAD_x86_64" => {
                    self.download_x86_64(value);
                }
                "MD5SUM_x86_64" => {
                    self.md5sum_x86_64(value);
                }
                "REQUIRES" => {
                    if self.requires(value).is_err() {
                        return Err(SboInfoError);
                    }
                }
                "MAINTAINER" => {
                    self.maintainer(value);
                }
                "EMAIL" => {
                    self.email(value);
                }
                _ => {
                    println!("Not found");
                }
            }
        }
        Ok(())
    }

    fn program_name(&mut self, contents: &str) {
        self.program_name = contents.to_string();
    }

    fn version(&mut self, contents: &str) {
        self.version = contents.to_string();
    }

    fn homepage(&mut self, contents: &str) {
        self.homepage = contents.to_string();
    }

    fn download(&mut self, contents: &str) {
        // TODO: Handle UNSUPPORTED
        let downloads = multiline_to_vec(contents);
        self.download = downloads;
    }

    fn md5sum(&mut self, contents: &str) {
        // TODO: Handle UNTESTED
        let checksums = multiline_to_vec(contents);
        self.md5sum = checksums;
    }

    fn download_x86_64(&mut self, contents: &str) {
        // TODO: Handle UNSUPPORTED
        let downloads = multiline_to_vec(contents);
        self.download_x86_64 = downloads;
    }

    fn md5sum_x86_64(&mut self, contents: &str) {
        // TODO: Handle UNTESTED
        let checksums = multiline_to_vec(contents);
        self.md5sum_x86_64 = checksums;
    }

    fn requires(&mut self, contents: &str) -> Result<(), RequiresError> {
        if contents.contains(r#"%README%"#) {
            return Err(RequiresError);
        }
        for requires in contents.split_whitespace() {
            self.requires.push(requires.to_string());
        }
        Ok(())
    }

    fn maintainer(&mut self, contents: &str) {
        self.maintainer = contents.to_string();
    }

    fn email(&mut self, contents: &str) {
        self.email = contents.to_string();
    }
}

fn multiline_to_vec(contents: &str) -> Vec<String> {
    let mut vec = Vec::new();
    for i in contents.lines() {
        vec.push(i.trim_end_matches(" \\").to_string());
    }
    vec
}

#[cfg(test)]
mod tests {
    use crate::info::SboInfo;

    #[test]
    fn valid_sboinfo_parse() {
        let example_info = r#"
PRGNAM="rdesktop"
VERSION="1.4.1"
HOMEPAGE="http://rdesktop.org"
DOWNLOAD="http://downloads.sourceforge.net/rdesktop/rdesktop-1.4.1.tar.gz"
MD5SUM="78dd2bae04edf1cb9f65c29930dcc993"
DOWNLOAD_x86_64=""
MD5SUM_x86_64=""
REQUIRES=""
MAINTAINER="Robby Workman"
EMAIL="rworkman@slackbuilds.org"
"#;
        let mut sb_info = SboInfo::new();
        let _ = sb_info.from_str(example_info);
        assert_eq!(sb_info.program_name, "rdesktop");
        assert_eq!(sb_info.version, "1.4.1");
        assert_eq!(sb_info.homepage, "http://rdesktop.org");
        assert_eq!(
            sb_info.download[0],
            "http://downloads.sourceforge.net/rdesktop/rdesktop-1.4.1.tar.gz"
        );
        assert_eq!(sb_info.md5sum[0], "78dd2bae04edf1cb9f65c29930dcc993");
        assert_eq!(sb_info.download_x86_64.len(), 0);
        assert_eq!(sb_info.md5sum_x86_64.len(), 0);
        assert_eq!(sb_info.requires.len(), 0);
        assert_eq!(sb_info.maintainer, "Robby Workman");
        assert_eq!(sb_info.email, "rworkman@slackbuilds.org");
    }

    #[test]
    fn valid_multiline_sbo_parse() {
        let example_info = r#"
PRGNAM="nordic-gtk-theme"
VERSION="2.2.0"
HOMEPAGE="https://github.com/EliverLara/Nordic"
DOWNLOAD="https://github.com/EliverLara/Nordic/releases/download/v2.2.0/Nordic.tar.xz \
https://github.com/EliverLara/Nordic/releases/download/v2.2.0/Nordic-standard-buttons.tar.xz \
https://github.com/EliverLara/Nordic/releases/download/v2.2.0/Nordic-Polar.tar.xz \
https://github.com/EliverLara/Nordic/releases/download/v2.2.0/Nordic-Polar-standard-buttons.tar.xz \
https://github.com/EliverLara/Nordic/releases/download/v2.2.0/Nordic-darker.tar.xz \
https://github.com/EliverLara/Nordic/releases/download/v2.2.0/Nordic-darker-standard-buttons.tar.xz \
https://github.com/EliverLara/Nordic/releases/download/v2.2.0/Nordic-bluish-accent.tar.xz \
https://github.com/EliverLara/Nordic/releases/download/v2.2.0/Nordic-bluish-accent-standard-buttons.tar.xz"
MD5SUM="a354aa55550f228223b81106dfeb918d \
0c0cb7ee25aafc9c192d8252896e20c3 \
f08a4361637a89923eb08eb4bd099751 \
bad92a77865fefc995aaee2f6e1dc7de \
5b0ee170958261a5ace50fb4f1a4cd7e \
6c301f501fed19c54a6ffbba67c5c43c \
4f6ef676a64a138135b9a666cf536ed8 \
8ed2902d0d0c63d095e1abdbf89aeb65"
DOWNLOAD_x86_64=""
MD5SUM_x86_64=""
REQUIRES=""
MAINTAINER="Marco Lavorini"
EMAIL="sbo.mlavorini@outlook.com"
"#;
        let mut sb_info = SboInfo::new();
        let _ = sb_info.from_str(example_info);
        assert_eq!(sb_info.download.len(), sb_info.md5sum.len());
        assert_eq!(sb_info.download.len(), 8);
    }

    #[test]
    fn multiple_requires_parse() {
        let example_info = r#"
PRGNAM="i3"
VERSION="4.22"
HOMEPAGE="https://www.i3wm.org"
DOWNLOAD="https://i3wm.org/downloads/i3-4.22.tar.xz"
MD5SUM="61c7787808344e2871079a9c93e751c2"
DOWNLOAD_x86_64=""
MD5SUM_x86_64=""
REQUIRES="dmenu libev xcb-util-xrm yajl perl-JSON-XS perl-AnyEvent"
MAINTAINER="Emmanuel N. Millan"
EMAIL="emmanueln@gmail.com"
"#;
        let mut sb_info = SboInfo::new();
        let _ = sb_info.from_str(example_info);
        assert_eq!(sb_info.requires.len(), 6);
    }

    #[test]
    fn requires_readme_exits() {
        let example_info = r#"
PRGNAM="i3"
VERSION="4.22"
HOMEPAGE="https://www.i3wm.org"
DOWNLOAD="https://i3wm.org/downloads/i3-4.22.tar.xz"
MD5SUM="61c7787808344e2871079a9c93e751c2"
DOWNLOAD_x86_64=""
MD5SUM_x86_64=""
REQUIRES="%README%"
MAINTAINER="Emmanuel N. Millan"
EMAIL="emmanueln@gmail.com"
"#;
        let mut sb_info = SboInfo::new();
        let result = sb_info.from_str(example_info);
        assert!(result.is_err());
    }
}

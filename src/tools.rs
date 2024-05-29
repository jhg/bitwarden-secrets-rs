use std::ffi::OsStr;
use std::io;
use std::process::Command;
use crate::helpers::BwObject;

pub(crate) struct BitwardenCli {
    session: String,
}

impl BitwardenCli {
    const BW_SESSION_ENV_VAR: &'static str = "BW_SESSION";

    fn command() -> Command {
        #[cfg(target_os = "windows")]
        return Command::new("bw.cmd");
        #[cfg(not(target_os = "windows"))]
        return Command::new("bw");
    }

    fn session_command(&self) -> Command {
        let mut command = Self::command();
        command.env(Self::BW_SESSION_ENV_VAR, &self.session);
        return command;
    }

    pub fn is_installed() -> bool {
        Self::command()
            .arg("--version")
            .output()
            .is_ok()
    }

    pub fn unlock<S: AsRef<OsStr>>(password: S) -> io::Result<Self> {
        Self::command()
            .arg("unlock")
            .arg("--raw")
            .arg(password)
            .output()
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
            })
            .map(|session| Self { session })
    }

    pub fn get_object<S: AsRef<OsStr>>(&self, object: BwObject, id: S) -> io::Result<String> {
        self.session_command()
            .arg("get")
            .arg(object.as_str())
            .arg(id)
            .output()
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
            })
    }
}

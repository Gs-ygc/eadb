use crate::{
    constants::to_rootfs_dir,
    exec::{check_call, check_output, run_pty},
    remote_op::RemoteOp,
};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Adb {
    serial: Option<String>,
    tcp_device: bool,
    usb_device: bool,
}

impl Adb {
    pub fn new(serial: Option<String>, tcp_device: bool, usb_device: bool) -> Self {
        Adb {
            serial,
            tcp_device,
            usb_device,
        }
    }

    fn get_cmd_prefix(&self) -> String {
        if let Some(serial) = &self.serial {
            return format!("adb -s {}", serial)
        }
        if self.tcp_device {
            "adb -e".to_string()
        } else if self.usb_device {
            "adb -d".to_string()
        } else {
            "adb".to_string()
        }
    }
}

impl RemoteOp for Adb {
    fn check_connection(&self) -> Result<()> {
        let code = run_pty(format!("{} get-state", self.get_cmd_prefix()))?;
        if code != 0 {
            return Err(anyhow::anyhow!("failed to connect to device"));
        }

        // todo: check multiple device connection

        let id = check_output(format!("{} shell id -u", self.get_cmd_prefix()))?;
        if id.trim() != "0" {
            return Err(anyhow::anyhow!("adb root is necessary!"));
        }
        Ok(())
    }

    fn shell(&self, cmd: &str) -> Result<()> {
        run_pty(format!("{} shell {}", self.get_cmd_prefix(), cmd)).map(|_| ())
    }

    fn check_call(&self, cmd: &str) -> Result<()> {
        check_call(format!("{} shell {}", self.get_cmd_prefix(), cmd))
    }

    fn push(&self, src: &str, dst: &str) -> Result<()> {
        check_call(format!(
            "{} push {} {}",
            self.get_cmd_prefix(),
            src,
            to_rootfs_dir(dst)
        ))
    }

    fn pull(&self, src: &str, dst: &str) -> Result<()> {
        check_call(format!(
            "{} pull {} {}",
            self.get_cmd_prefix(),
            to_rootfs_dir(src),
            dst
        ))
    }

    fn check_output(&self, cmd: &str) -> Result<String> {
        check_output(cmd)
    }
}

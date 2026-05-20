use anyhow::{Context, Result};
use std::ffi::CString;
use std::fmt;
use std::fs;
use std::os::fd::RawFd;
use std::path::{Path, PathBuf};

const HDIO_GET_IDENTITY: libc::c_ulong = 0x030d;
const BLKGETSIZE64: libc::c_ulong = 0x8008_1272;
const BLKSSZGET: libc::c_ulong = 0x1268;
const BLKROGET: libc::c_ulong = 0x125e;

pub struct MachineInspect {
    ssd_info: Vec<SsdInfo>,
}

#[derive(Default)]
pub struct SsdInfo {
    name: String,
    device_path: PathBuf,
    model: Option<String>,
    serial: Option<String>,
    firmware: Option<String>,
    capacity_bytes: Option<u64>,
    logical_block_size: Option<u32>,
    rotational: Option<bool>,
    read_only: Option<bool>,
    transport: Option<String>,
    health: Option<String>,
    ioctl_notes: Vec<String>,
}

struct Fd(RawFd);

impl Drop for Fd {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

impl MachineInspect {
    pub fn new() -> Self {
        Self {
            ssd_info: Vec::new(),
        }
    }

    pub fn inspect(&mut self) -> Result<()> {
        self.ssd_info.clear();

        for entry in fs::read_dir("/sys/block").context("read /sys/block")? {
            let entry = entry.context("read /sys/block entry")?;
            let name = entry.file_name().to_string_lossy().into_owned();

            if !is_disk_name(&name) {
                continue;
            }

            self.ssd_info.push(inspect_disk(&name));
        }

        Ok(())
    }

    pub fn display(&self) {
        println!("Machine Inspection:");

        if self.ssd_info.is_empty() {
            println!("No SSD/block disk devices found.");
            return;
        }

        for disk in &self.ssd_info {
            println!();
            println!("Device: {}", disk.device_path.display());
            println!("  Name: {}", disk.name);
            println!("  Model: {}", display_opt(&disk.model));
            println!("  Serial: {}", display_opt(&disk.serial));
            println!("  Firmware: {}", display_opt(&disk.firmware));
            println!("  Capacity: {}", format_capacity(disk.capacity_bytes));
            println!(
                "  Logical block size: {}",
                disk.logical_block_size
                    .map(|value| format!("{value} bytes"))
                    .unwrap_or_else(|| "unknown".to_string())
            );
            println!("  Rotational: {}", display_bool(disk.rotational));
            println!("  Read-only: {}", display_bool(disk.read_only));
            println!("  Transport: {}", display_opt(&disk.transport));
            println!("  Health: {}", display_opt(&disk.health));

            if !disk.ioctl_notes.is_empty() {
                println!("  Notes:");
                for note in &disk.ioctl_notes {
                    println!("    - {note}");
                }
            }
        }
    }
}

pub fn inspect_and_display() -> Result<()> {
    let mut inspect = MachineInspect::new();
    inspect.inspect()?;
    inspect.display();
    Ok(())
}

fn inspect_disk(name: &str) -> SsdInfo {
    let device_path = PathBuf::from(format!("/dev/{name}"));
    let sys_block = PathBuf::from(format!("/sys/block/{name}"));

    let mut info = SsdInfo {
        name: name.to_string(),
        device_path,
        model: read_trimmed(sys_block.join("device/model")),
        serial: read_trimmed(sys_block.join("device/serial")),
        firmware: read_trimmed(sys_block.join("device/firmware_rev")),
        capacity_bytes: sysfs_capacity_bytes(&sys_block),
        logical_block_size: read_trimmed(sys_block.join("queue/logical_block_size"))
            .and_then(|value| value.parse().ok()),
        transport: read_trimmed(sys_block.join("device/transport")),
        rotational: read_trimmed(sys_block.join("queue/rotational")).map(|value| value == "1"),
        ..SsdInfo::default()
    };

    let fd = match open_read_only(&info.device_path) {
        Ok(fd) => fd,
        Err(err) => {
            info.ioctl_notes.push(format!(
                "Could not open {} for ioctl: {err:#}",
                info.device_path.display()
            ));
            info.health = Some(health_summary(&info));
            return info;
        }
    };

    match ioctl_ata_identity(fd.0) {
        Ok(identity) => {
            info.model = Some(identity.model);
            info.serial = Some(identity.serial);
            info.firmware = Some(identity.firmware);
        }
        Err(err) => {
            info.ioctl_notes
                .push(format!("HDIO_GET_IDENTITY unavailable: {err}"));
        }
    }

    match ioctl_u64(fd.0, BLKGETSIZE64) {
        Ok(value) => info.capacity_bytes = Some(value),
        Err(err) => info
            .ioctl_notes
            .push(format!("BLKGETSIZE64 unavailable: {err}")),
    }

    match ioctl_i32(fd.0, BLKSSZGET) {
        Ok(value) => info.logical_block_size = u32::try_from(value).ok(),
        Err(err) => info
            .ioctl_notes
            .push(format!("BLKSSZGET unavailable: {err}")),
    }

    match ioctl_i32(fd.0, BLKROGET) {
        Ok(value) => info.read_only = Some(value != 0),
        Err(err) => info
            .ioctl_notes
            .push(format!("BLKROGET unavailable: {err}")),
    }

    info.health = Some(health_summary(&info));

    info
}

fn open_read_only(path: &Path) -> Result<Fd> {
    let path = CString::new(path.as_os_str().as_encoded_bytes())
        .with_context(|| format!("device path contains NUL: {}", path.display()))?;
    let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY | libc::O_CLOEXEC) };

    if fd < 0 {
        Err(std::io::Error::last_os_error()).context("open block device")
    } else {
        Ok(Fd(fd))
    }
}

fn ioctl_u64(fd: RawFd, request: libc::c_ulong) -> std::io::Result<u64> {
    let mut value = 0_u64;
    let rc = unsafe { libc::ioctl(fd, request, &mut value) };
    if rc < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(value)
    }
}

fn ioctl_i32(fd: RawFd, request: libc::c_ulong) -> std::io::Result<i32> {
    let mut value = 0_i32;
    let rc = unsafe { libc::ioctl(fd, request, &mut value) };
    if rc < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(value)
    }
}

fn ioctl_ata_identity(fd: RawFd) -> std::io::Result<AtaIdentity> {
    let mut data = [0_u16; 256];
    let rc = unsafe { libc::ioctl(fd, HDIO_GET_IDENTITY, data.as_mut_ptr()) };

    if rc < 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(AtaIdentity {
        serial: ata_string(&data[10..20]),
        firmware: ata_string(&data[23..27]),
        model: ata_string(&data[27..47]),
    })
}

struct AtaIdentity {
    serial: String,
    firmware: String,
    model: String,
}

fn ata_string(words: &[u16]) -> String {
    let mut bytes = Vec::with_capacity(words.len() * 2);
    for word in words {
        bytes.push((word >> 8) as u8);
        bytes.push((word & 0xff) as u8);
    }

    String::from_utf8_lossy(&bytes).trim().to_string()
}

fn read_trimmed(path: impl AsRef<Path>) -> Option<String> {
    let value = fs::read_to_string(path).ok()?;
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn sysfs_capacity_bytes(sys_block: &Path) -> Option<u64> {
    let sectors = read_trimmed(sys_block.join("size"))?.parse::<u64>().ok()?;
    Some(sectors * 512)
}

fn is_disk_name(name: &str) -> bool {
    (name.starts_with("sd") && name.len() == 3)
        || (name.starts_with("nvme") && name.contains('n') && !name.contains('p'))
        || (name.starts_with("vd") && name.len() == 3)
        || (name.starts_with("xvd") && name.len() == 4)
}

fn health_summary(info: &SsdInfo) -> String {
    let mut parts = Vec::new();

    match info.rotational {
        Some(false) => parts.push("non-rotational media"),
        Some(true) => parts.push("rotational media"),
        None => parts.push("rotation unknown"),
    }

    match info.read_only {
        Some(false) => parts.push("writable"),
        Some(true) => parts.push("read-only"),
        None => parts.push("read-only state unknown"),
    }

    parts.join(", ")
}

fn display_opt(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("unknown")
}

fn display_bool(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "unknown",
    }
}

fn format_capacity(value: Option<u64>) -> String {
    let Some(bytes) = value else {
        return "unknown".to_string();
    };

    ByteSize(bytes).to_string()
}

struct ByteSize(u64);

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0;
        let gib = bytes as f64 / 1024_f64.powi(3);
        let gb = bytes as f64 / 1000_f64.powi(3);
        write!(f, "{bytes} bytes ({gib:.2} GiB / {gb:.2} GB)")
    }
}

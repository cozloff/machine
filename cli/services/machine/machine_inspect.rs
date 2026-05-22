use anyhow::{Context, Result};
use std::ffi::CString;
use std::fs;
use std::os::fd::RawFd;
use std::path::{Path, PathBuf};
use crate::services::display::spec_fmt::*;

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
    sysfs_path: PathBuf,
    model: Option<String>,
    vendor: Option<String>,
    serial: Option<String>,
    firmware: Option<String>,
    capacity_bytes: Option<u64>,
    logical_block_size: Option<u32>,
    physical_block_size: Option<u32>,
    minimum_io_size: Option<u32>,
    optimal_io_size: Option<u32>,
    rotational: Option<bool>,
    read_only: Option<bool>,
    transport: Option<String>,
    device_type: Option<String>,
    subsystem: Option<String>,
    driver: Option<String>,
    device_link: Option<PathBuf>,
    discard_max_bytes: Option<u64>,
    discard_granularity: Option<u64>,
    max_sectors_kb: Option<u64>,
    nr_requests: Option<u64>,
    scheduler: Option<String>,
    partitions: Vec<PartitionInfo>,
    health: Option<String>,
    ioctl_notes: Vec<String>,
}

#[derive(Default)]
struct PartitionInfo {
    name: String,
    device_path: PathBuf,
    size_bytes: Option<u64>,
    filesystem: Option<String>,
    label: Option<String>,
    uuid: Option<String>,
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
            println!("  Sysfs: {}", disk.sysfs_path.display());
            println!("  Model: {}", SpecFmt::opt(&disk.model));
            println!("  Vendor: {}", SpecFmt::opt(&disk.vendor));
            println!("  Serial: {}", SpecFmt::opt(&disk.serial));
            println!("  Firmware: {}", SpecFmt::opt(&disk.firmware));
            println!("  Capacity: {}", SpecFmt::capacity(disk.capacity_bytes));
            println!(
                "  Logical block size: {}",
                SpecFmt::bytes_u32(disk.logical_block_size)
            );
            println!(
                "  Physical block size: {}",
                SpecFmt::bytes_u32(disk.physical_block_size)
            );
            println!(
                "  Minimum I/O size: {}",
                SpecFmt::bytes_u32(disk.minimum_io_size)
            );
            println!(
                "  Optimal I/O size: {}",
                SpecFmt::bytes_u32(disk.optimal_io_size)
            );
            println!("  Rotational: {}", SpecFmt::bool(disk.rotational));
            println!("  Read-only: {}", SpecFmt::bool(disk.read_only));
            println!("  Transport: {}", SpecFmt::opt(&disk.transport));
            println!("  Device type: {}", SpecFmt::opt(&disk.device_type));
            println!("  Subsystem: {}", SpecFmt::opt(&disk.subsystem));
            println!("  Driver: {}", SpecFmt::opt(&disk.driver));
            println!("  Device link: {}", SpecFmt::path(&disk.device_link));
            println!(
                "  Discard/TRIM max: {}",
                SpecFmt::capacity(disk.discard_max_bytes)
            );
            println!(
                "  Discard/TRIM granularity: {}",
                SpecFmt::capacity(disk.discard_granularity)
            );
            println!("  Max sectors: {}", SpecFmt::kib(disk.max_sectors_kb));
            println!("  Queue requests: {}", SpecFmt::u64(disk.nr_requests));
            println!("  Scheduler: {}", SpecFmt::opt(&disk.scheduler));
            println!("  Health: {}", SpecFmt::opt(&disk.health));

            if !disk.partitions.is_empty() {
                println!("  Partitions:");
                for partition in &disk.partitions {
                    println!(
                        "    - {} {} fs={} label={} uuid={}",
                        partition.device_path.display(),
                        SpecFmt::capacity(partition.size_bytes),
                        SpecFmt::opt(&partition.filesystem),
                        SpecFmt::opt(&partition.label),
                        SpecFmt::opt(&partition.uuid),
                    );
                }
            }

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
        sysfs_path: sys_block.clone(),
        model: read_trimmed(sys_block.join("device/model")),
        vendor: read_trimmed(sys_block.join("device/vendor")),
        serial: read_trimmed(sys_block.join("device/serial")),
        firmware: read_trimmed(sys_block.join("device/firmware_rev")),
        capacity_bytes: sysfs_capacity_bytes(&sys_block),
        logical_block_size: read_trimmed(sys_block.join("queue/logical_block_size"))
            .and_then(|value| value.parse().ok()),
        physical_block_size: read_trimmed(sys_block.join("queue/physical_block_size"))
            .and_then(|value| value.parse().ok()),
        minimum_io_size: read_trimmed(sys_block.join("queue/minimum_io_size"))
            .and_then(|value| value.parse().ok()),
        optimal_io_size: read_trimmed(sys_block.join("queue/optimal_io_size"))
            .and_then(|value| value.parse().ok()),
        transport: read_trimmed(sys_block.join("device/transport")),
        device_type: read_trimmed(sys_block.join("device/type")),
        subsystem: link_target_name(sys_block.join("device/subsystem")),
        driver: link_target_name(sys_block.join("device/driver")),
        device_link: fs::read_link(sys_block.join("device")).ok(),
        discard_max_bytes: read_trimmed(sys_block.join("queue/discard_max_bytes"))
            .and_then(|value| value.parse().ok()),
        discard_granularity: read_trimmed(sys_block.join("queue/discard_granularity"))
            .and_then(|value| value.parse().ok()),
        max_sectors_kb: read_trimmed(sys_block.join("queue/max_sectors_kb"))
            .and_then(|value| value.parse().ok()),
        nr_requests: read_trimmed(sys_block.join("queue/nr_requests"))
            .and_then(|value| value.parse().ok()),
        scheduler: read_trimmed(sys_block.join("queue/scheduler")),
        rotational: read_trimmed(sys_block.join("queue/rotational")).map(|value| value == "1"),
        partitions: inspect_partitions(&sys_block, name),
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

fn link_target_name(path: impl AsRef<Path>) -> Option<String> {
    fs::read_link(path)
        .ok()?
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
}

fn sysfs_capacity_bytes(sys_block: &Path) -> Option<u64> {
    let sectors = read_trimmed(sys_block.join("size"))?.parse::<u64>().ok()?;
    Some(sectors * 512)
}

fn inspect_partitions(sys_block: &Path, disk_name: &str) -> Vec<PartitionInfo> {
    let Ok(entries) = fs::read_dir(sys_block) else {
        return Vec::new();
    };

    let mut partitions = Vec::new();

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !is_partition_name(disk_name, &name) {
            continue;
        }

        let partition_path = entry.path();
        let device_path = PathBuf::from(format!("/dev/{name}"));
        let by_uuid = PathBuf::from("/dev/disk/by-uuid");
        let by_label = PathBuf::from("/dev/disk/by-label");

        partitions.push(PartitionInfo {
            name,
            device_path,
            size_bytes: sysfs_capacity_bytes(&partition_path),
            filesystem: read_trimmed(partition_path.join("partition"))
                .map(|_| "partition".to_string()),
            label: find_dev_disk_link(&by_label, &partition_path),
            uuid: find_dev_disk_link(&by_uuid, &partition_path),
        });
    }

    partitions.sort_by(|left, right| left.name.cmp(&right.name));
    partitions
}

fn is_disk_name(name: &str) -> bool {
    (name.starts_with("sd") && name.len() == 3)
        || (name.starts_with("nvme") && name.contains('n') && !name.contains('p'))
        || (name.starts_with("vd") && name.len() == 3)
        || (name.starts_with("xvd") && name.len() == 4)
}

fn is_partition_name(disk_name: &str, name: &str) -> bool {
    if disk_name.starts_with("nvme") {
        return name.starts_with(disk_name) && name[disk_name.len()..].starts_with('p');
    }

    name.starts_with(disk_name) && name.len() > disk_name.len()
}

fn find_dev_disk_link(link_dir: &Path, partition_path: &Path) -> Option<String> {
    let entries = fs::read_dir(link_dir).ok()?;
    let partition_name = partition_path.file_name()?;

    for entry in entries.flatten() {
        let target = fs::read_link(entry.path()).ok()?;
        if target.file_name() == Some(partition_name) {
            return Some(entry.file_name().to_string_lossy().into_owned());
        }
    }

    None
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
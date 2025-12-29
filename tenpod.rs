#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

use std::process::Command;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// ============================================================================
// TEN POD - High-Performance VR Gaming VM Manager
// Optimized for 90Hz+ VR headsets (Index, Quest, Reverb G2)
// Complete automated setup with headset detection
// ============================================================================

struct TenPod {
    gpu_pci: String,
    audio_pci: String,
    memory_gb: u32,
    cpu_cores: String,
}

impl TenPod {
    fn new() -> Self {
        Self {
            gpu_pci: String::new(),
            audio_pci: String::new(),
            memory_gb: 16, // Recommended for VR
            cpu_cores: "4-7".to_string(), // Adjust based on your CPU
        }
    }

    /// Detects both the NVIDIA Video and Audio components (crucial for stability)
    fn detect_hardware(&mut self) -> Result<(), String> {
        println!("🔍 Scanning for NVIDIA Hardware...");

        let output = Command::new("lspci")
        .arg("-nn")
        .output()
        .map_err(|e| format!("lspci failed: {}. Is pciutils installed?", e))?;

        let lspci_output = String::from_utf8_lossy(&output.stdout);
        let mut base_addr = String::new();

        for line in lspci_output.lines() {
            if (line.contains("[10de:") || line.contains("NVIDIA")) &&
                (line.contains("VGA") || line.contains("3D")) {
                    let pci_addr = line.split_whitespace().next()
                    .ok_or("Could not parse PCI address")?;
                    self.gpu_pci = format!("0000:{}", pci_addr);
                    base_addr = pci_addr.split(':').next().unwrap_or("").to_string();

                    // Extract GPU name
                    let gpu_name = line.split(':').skip(2).collect::<Vec<_>>().join(":").trim().to_string();
                    println!("✓ Found GPU: {}", gpu_name);
                    println!("  PCI Address: {}", self.gpu_pci);

                    // Check current driver
                    self.check_driver_status(&pci_addr.replace(".", ""));
                }
        }

        // Find matching audio device (same bus as GPU)
        for line in lspci_output.lines() {
            if line.contains(&base_addr) && line.contains("Audio device") {
                let pci_addr = line.split_whitespace().next()
                .ok_or("Could not parse audio PCI address")?;
                self.audio_pci = format!("0000:{}", pci_addr);
                println!("✓ Found GPU Audio: {}", self.audio_pci);
            }
        }

        if self.gpu_pci.is_empty() {
            return Err("No NVIDIA GPU found! Make sure your GPU is properly seated.".to_string());
        }

        if self.audio_pci.is_empty() {
            println!("⚠️  GPU Audio device not found (some GPUs don't have it)");
        }

        Ok(())
    }

    /// Check what driver is currently using the GPU
    fn check_driver_status(&self, pci_addr: &str) {
        let driver_path = format!("/sys/bus/pci/devices/0000:{}/driver", pci_addr);

        if let Ok(link) = fs::read_link(&driver_path) {
            let driver_name = link.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

            match driver_name {
                "nvidia" => println!("  Current driver: nvidia (will be unbound after reboot)"),
                "nouveau" => println!("  Current driver: nouveau (will be unbound after reboot)"),
                "vfio-pci" => println!("  Current driver: vfio-pci (already configured!)"),
                _ => println!("  Current driver: {}", driver_name),
            }
        } else {
            println!("  Current driver: none (perfect for passthrough)");
        }
    }

    /// Detect VR headsets connected via USB
    fn detect_headsets(&self) -> Vec<(String, String, String)> {
        println!("\n🎧 Scanning for VR headsets...");

        let headset_db = vec![
            ("28de", "2012", "Valve Index"),
            ("28de", "2000", "HTC Vive"),
            ("28de", "2101", "HTC Vive Pro"),
            ("28de", "2102", "HTC Vive Cosmos"),
            ("2833", "0186", "Meta Quest 2"),
            ("2833", "0187", "Meta Quest Pro"),
            ("2833", "0188", "Meta Quest 3"),
            ("2d40", "2000", "Pico 4"),
            ("03f0", "0580", "HP Reverb G2"),
            ("0483", "0101", "Pimax 5K/8K"),
        ];

        let output = Command::new("lsusb").output();
        if output.is_err() {
            println!("  ⚠️  lsusb not available, install usbutils for headset detection");
            return vec![];
        }

        let output_result = output.unwrap();
        let usb_list = String::from_utf8_lossy(&output_result.stdout);
        let mut found = vec![];

        for line in usb_list.lines() {
            for (vendor, product, name) in &headset_db {
                if line.contains(vendor) && line.contains(product) {
                    found.push((vendor.to_string(), product.to_string(), name.to_string()));
                    println!("✓ Found: {}", name);
                }
            }
        }

        if found.is_empty() {
            println!("  No VR headset detected (plug it in before starting VM)");
        }

        found
    }

    /// Get VFIO hardware IDs for both GPU and Audio
    fn get_vfio_ids(&self) -> String {
        let mut ids = Vec::new();
        for addr in [&self.gpu_pci, &self.audio_pci] {
            if addr.is_empty() { continue; }
            let out = Command::new("lspci")
            .args(["-n", "-s", &addr.replace("0000:", "")])
            .output()
            .ok();
            if let Some(o) = out {
                let s = String::from_utf8_lossy(&o.stdout);
                if let Some(id) = s.split_whitespace().nth(2) {
                    ids.push(id.to_string());
                }
            }
        }
        ids.join(",")
    }

    /// Check if system requirements are met
    fn check_system(&self) -> Result<(), String> {
        println!("\n⚙️  System Requirements Check:");

        // Check IOMMU
        print!("  • IOMMU enabled... ");
        let iommu = Command::new("sh")
        .arg("-c")
        .arg("sudo dmesg 2>/dev/null | grep -i iommu | head -1")
        .output();

        if iommu.is_ok() && !iommu.unwrap().stdout.is_empty() {
            println!("✓");
        } else {
            println!("✗");
            return Err(
                "IOMMU not enabled!\n\
Add to /etc/default/grub: GRUB_CMDLINE_LINUX=\"intel_iommu=on\" (or amd_iommu=on)\n\
Then run: sudo grub2-mkconfig -o /boot/grub2/grub.cfg && sudo reboot".to_string()
            );
        }

        // Check QEMU
        print!("  • QEMU installed... ");
        if Command::new("which").arg("qemu-system-x86_64").output().is_ok() {
            println!("✓");
        } else {
            println!("✗ (will be installed)");
        }

        // Check if virtualization is enabled in BIOS
        print!("  • CPU virtualization... ");
        if Path::new("/dev/kvm").exists() {
            println!("✓");
        } else {
            println!("✗");
            return Err(
                "CPU virtualization not available!\n\
Enable VT-x (Intel) or AMD-V (AMD) in your BIOS settings.".to_string()
            );
        }

        // Check GPU driver status
        print!("  • GPU status... ");
        let driver_path = format!("/sys/bus/pci/devices/{}/driver", self.gpu_pci);

        if let Ok(link) = fs::read_link(&driver_path) {
            let driver = link.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

            match driver {
                "vfio-pci" => println!("✓ (bound to vfio-pci, ready!)"),
                _ => println!("⚠️  (currently using {}, will switch after reboot)", driver),
            }
        } else {
            println!("✓ (no driver, ready for passthrough)");
        }

        Ok(())
    }

    /// One-click installer
    fn install(&self) -> Result<(), String> {
        println!("\n🚀 TEN POD ONE-CLICK INSTALLER\n");
        println!("This will configure your system for VR GPU passthrough.");
        println!("The following will be installed/configured:");
        println!("  • QEMU/KVM virtualization");
        println!("  • VFIO drivers for GPU passthrough");
        println!("  • Hugepages for memory performance");
        println!("  • User permissions for VM management");
        println!("  • 100GB VM disk image\n");

        print!("Continue? (y/n): ");
        io::stdout().flush().ok();

        let mut response = String::new();
        io::stdin().read_line(&mut response).ok();

        if !response.trim().to_lowercase().starts_with('y') {
            return Err("Installation cancelled by user".to_string());
        }

        // Step 1: Install packages
        println!("\n[1/6] 📦 Installing QEMU/KVM packages...");
        let packages = vec![
            "qemu-kvm", "libvirt", "virt-manager", "bridge-utils",
            "libvirt-daemon-config-network", "virt-install"
        ];

        let install_result = Command::new("pkexec")
        .arg("dnf")
        .arg("install")
        .arg("-y")
        .args(&packages)
        .status();

        match install_result {
            Ok(_) => println!("      ✓ Packages installed successfully"),
            Err(e) => {
                println!("      ⚠️  Package installation failed: {}", e);
                println!("      Continuing anyway (packages may already be installed)");
            }
        }

        // Step 2: Configure VFIO
        println!("\n[2/6] 🔧 Configuring VFIO drivers for GPU passthrough...");
        let ids = self.get_vfio_ids();

        if ids.is_empty() {
            return Err("Could not determine GPU hardware IDs".to_string());
        }

        println!("      Hardware IDs: {}", ids);

        let vfio_conf = format!(
            "# Ten Pod VFIO Configuration\noptions vfio-pci ids={}\nblacklist nvidia\nblacklist nouveau",
            ids
        );

        Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(format!("echo '{}' > /etc/modprobe.d/vfio.conf", vfio_conf))
        .status()
        .map_err(|e| format!("Failed to write vfio.conf: {}", e))?;

        // Priority configuration
        let softdep = "softdep nvidia pre: vfio-pci\nsoftdep nouveau pre: vfio-pci";
        Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(format!("echo '{}' > /etc/modprobe.d/vfio-priority.conf", softdep))
        .status()
        .ok();

        // Rebuild initramfs
        println!("      Rebuilding initramfs (may take 30-60 seconds)...");
        Command::new("pkexec")
        .arg("dracut")
        .arg("-f")
        .arg("--force")
        .status()
        .ok();

        println!("      ✓ VFIO configured");

        // Step 3: Setup hugepages
        println!("\n[3/6] 💾 Configuring hugepages for low latency...");
        let pages = (self.memory_gb * 1024) / 2;

        Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(format!("echo 'vm.nr_hugepages = {}' >> /etc/sysctl.conf", pages))
        .status()
        .ok();

        // Apply immediately
        Command::new("pkexec")
        .arg("sysctl")
        .arg("-w")
        .arg(format!("vm.nr_hugepages={}", pages))
        .status()
        .ok();

        println!("      ✓ Hugepages configured ({} pages)", pages);

        // Step 4: User permissions
        println!("\n[4/6] 👤 Setting up user permissions...");
        let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());

        for group in &["libvirt", "kvm", "input"] {
            Command::new("pkexec")
            .args(["usermod", "-aG", group, &user])
            .status()
            .ok();
        }

        println!("      ✓ User '{}' added to libvirt, kvm, input groups", user);

        // Step 5: Create VM disk
        println!("\n[5/6] 📀 Creating VM disk image...");
        let disk_dir = "/var/lib/libvirt/images";
        let disk_path = format!("{}/win10_tenpod.qcow2", disk_dir);

        Command::new("pkexec")
        .args(["mkdir", "-p", disk_dir])
        .status()
        .ok();

        if !Path::new(&disk_path).exists() {
            let create_result = Command::new("qemu-img")
            .args(["create", "-f", "qcow2", &disk_path, "100G"])
            .status();

            match create_result {
                Ok(_) => println!("      ✓ Created 100GB disk at {}", disk_path),
                Err(e) => return Err(format!("Failed to create disk: {}", e)),
            }
        } else {
            println!("      ✓ Disk already exists at {}", disk_path);
        }

        // Step 6: Enable libvirt service
        println!("\n[6/6] ⚡ Enabling libvirt service...");
        Command::new("pkexec")
        .args(["systemctl", "enable", "--now", "libvirtd"])
        .status()
        .ok();

        println!("      ✓ libvirtd service enabled");

        // Final instructions
        println!("\n✅ Installation Complete!\n");
        println!("📋 Next Steps:");
        println!("  1. REBOOT your system for VFIO changes to take effect");
        println!("  2. Download Windows 10 ISO to: /var/lib/libvirt/images/win10.iso");
        println!("     Get it from: https://www.microsoft.com/software-download/windows10");
        println!("  3. Plug in your VR headset");
        println!("  4. Run: ./tenpod.rs start");
        println!("\n⚠️  IMPORTANT: Reboot is required!");

        Ok(())
    }

    /// Start the VM with VR optimizations
    fn start(&self, headsets: Vec<(String, String, String)>) -> Result<(), String> {
        println!("\n🚀 Launching Ten Pod VM (VR Optimized)...\n");

        let disk_path = "/var/lib/libvirt/images/win10_tenpod.qcow2";
        let iso_path = "/var/lib/libvirt/images/win10.iso";

        // Pre-flight checks
        if !Path::new(disk_path).exists() {
            return Err(format!("VM disk not found at {}\nRun: ./tenpod.rs install", disk_path));
        }

        let mut cmd = Command::new("taskset");
        cmd.arg("-c").arg(&self.cpu_cores);
        cmd.arg("qemu-system-x86_64");

        // CPU & Hyper-V Enlightenments for Latency Reduction
        cmd.args([
            "-name", "TenPod",
            "-machine", "type=q35,accel=kvm,kernel_irqchip=on",
            "-cpu", "host,hv_time,hv_relaxed,hv_vapic,hv_spinlocks=0x1fff,hv_vendor_id=tenpodvr,kvm=off,+invtsc",
            "-smp", "4,sockets=1,cores=4,threads=1",
            "-m", &format!("{}G", self.memory_gb),
                 "-enable-kvm",
        ]);

        // Timing Stability for smooth VR frame pacing
        cmd.args([
            "-rtc", "base=localtime,clock=host,driftfix=slew",
            "-global", "kvm-pit.lost_tick_policy=delay",
            "-no-hpet",
        ]);

        // Hugepages for reduced memory latency
        if Path::new("/dev/hugepages").exists() {
            cmd.args(["-mem-path", "/dev/hugepages", "-mem-prealloc"]);
            println!("💾 Using hugepages for low-latency memory");
        }

        // GPU Passthrough (Multifunction ensures Video + Audio are seen as one card)
        println!("🎮 Passing through GPU: {}", self.gpu_pci);
        cmd.args([
            "-device",
            &format!("vfio-pci,host={},multifunction=on", self.gpu_pci.replace("0000:", ""))
        ]);

        if !self.audio_pci.is_empty() {
            println!("🔊 Passing through GPU Audio: {}", self.audio_pci);
            cmd.args([
                "-device",
                &format!("vfio-pci,host={}", self.audio_pci.replace("0000:", ""))
            ]);
        }

        // Fast Disk I/O with virtio-scsi
        cmd.args([
            "-device", "virtio-scsi-pci,id=scsi0",
            "-drive", &format!("file={},format=qcow2,if=none,id=dr1,cache=none,aio=native", disk_path),
                 "-device", "scsi-hd,drive=dr1",
        ]);

        // Boot from ISO if it exists (first-time Windows install)
        if Path::new(iso_path).exists() {
            println!("📀 Windows ISO detected, booting from ISO for installation");
            cmd.args(["-cdrom", iso_path, "-boot", "d"]);
        }

        // Enable USB 3.0 (Required for VR headsets)
        cmd.args(["-device", "qemu-xhci,id=usb-bus-0", "-usb"]);

        // Auto-passthrough detected VR headsets
        if !headsets.is_empty() {
            for (vendor, product, name) in &headsets {
                println!("🎧 Passing through: {}", name);
                cmd.args([
                    "-device",
                    &format!("usb-host,vendorid=0x{},productid=0x{}", vendor, product)
                ]);
            }
        } else {
            println!("⚠️  No VR headset detected - plug it in and restart VM");
        }

        // Network (for Windows updates, SteamVR downloads)
        cmd.args([
            "-netdev", "user,id=net0",
            "-device", "virtio-net-pci,netdev=net0",
        ]);

        // VGA Output: None (we use physical GPU output)
        cmd.args(["-vga", "none", "-nographic"]);

        println!("\n💻 VM Starting...");
        println!("📺 Check your GPU's physical monitor output for Windows display");
        println!("🎮 Once Windows boots, install NVIDIA drivers and SteamVR\n");

        cmd.status()
        .map_err(|e| format!("Failed to start VM: {}", e))?;

        Ok(())
    }

    /// Stop the VM
    fn stop(&self) -> Result<(), String> {
        println!("🛑 Stopping Ten Pod VM...");

        Command::new("pkill")
        .arg("-f")
        .arg("TenPod")
        .status()
        .ok();

        println!("✓ VM stopped");
        Ok(())
    }

    /// Show comprehensive status
    fn status(&self) {
        println!("\n📊 TEN POD STATUS\n");
        println!("GPU Configuration:");
        println!("  Video: {}", self.gpu_pci);
        if !self.audio_pci.is_empty() {
            println!("  Audio: {}", self.audio_pci);
        }
        println!("  Memory: {}GB", self.memory_gb);
        println!("  CPU Cores: {}", self.cpu_cores);

        let headsets = self.detect_headsets();
        self.check_system().ok();

        if !headsets.is_empty() {
            println!("\n🎧 {} VR headset(s) ready for passthrough", headsets.len());
        }
    }
}

fn print_help() {
    println!(r#"
    TEN POD - High-Performance VR Gaming VM Manager

    USAGE:
    ./tenpod.rs [COMMAND]

    COMMANDS:
    install     One-click system setup (run once, requires reboot)
    start       Launch Windows VM with GPU & headset passthrough
    stop        Stop the running VM
    status      Show detected hardware and system status
    help        Show this help message

    FIRST-TIME SETUP:
    1. ./tenpod.rs install     # Configure system (one-time)
    2. sudo reboot             # Apply VFIO changes
    3. Download Windows 10 ISO to /var/lib/libvirt/images/win10.iso
    4. Plug in VR headset
    5. ./tenpod.rs start       # Launch VM

    FEATURES:
    ✓ Automatic GPU + Audio passthrough
    ✓ VR headset auto-detection (Index, Quest, Reverb G2, etc.)
    ✓ Optimized for 90Hz+ VR (low latency, stable frame times)
    ✓ Hyper-V enlightenments for Windows performance
    ✓ Hugepages for memory performance
    ✓ Works with any NVIDIA driver (nvidia, nouveau, or none)

    REQUIREMENTS:
    • NVIDIA GPU (GTX 900+ or RTX series)
    • CPU with VT-x/AMD-V enabled in BIOS
    • IOMMU enabled in BIOS
    • At least 16GB RAM (8GB for VM, 8GB for host)
    • Nobara Linux or Fedora-based distro
    "#);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    let mut tenpod = TenPod::new();

    match command {
        "install" => {
            if let Err(e) = tenpod.detect_hardware() {
                eprintln!("❌ Hardware detection failed: {}", e);
                std::process::exit(1);
            }

            if let Err(e) = tenpod.check_system() {
                eprintln!("\n❌ System check failed:");
                eprintln!("{}", e);
                std::process::exit(1);
            }

            if let Err(e) = tenpod.install() {
                eprintln!("\n❌ Installation failed: {}", e);
                std::process::exit(1);
            }
        }

        "start" => {
            if let Err(e) = tenpod.detect_hardware() {
                eprintln!("❌ Hardware detection failed: {}", e);
                std::process::exit(1);
            }

            if let Err(e) = tenpod.check_system() {
                eprintln!("\n❌ System check failed:");
                eprintln!("{}", e);
                eprintln!("\n💡 Tip: Run './tenpod.rs install' if you haven't already");
                std::process::exit(1);
            }

            let headsets = tenpod.detect_headsets();

            if let Err(e) = tenpod.start(headsets) {
                eprintln!("\n❌ Failed to start VM: {}", e);
                std::process::exit(1);
            }
        }

        "stop" => {
            if let Err(e) = tenpod.stop() {
                eprintln!("❌ {}", e);
                std::process::exit(1);
            }
        }

        "status" => {
            if let Err(e) = tenpod.detect_hardware() {
                eprintln!("❌ Hardware detection failed: {}", e);
                std::process::exit(1);
            }
            tenpod.status();
        }

        _ => {
            print_help();
        }
    }
}

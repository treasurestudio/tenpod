# Ten Pod 🎮

**Universal VR Gaming VM Manager with GPU Passthrough**

Turn any VR headset + NVIDIA GPU into a high-performance Windows gaming setup on Linux. Zero lag, native GPU speeds, automatic headset detection.

---

## ⚠️ IMPORTANT: System Requirements

### **Disk Space Requirements**
- **Minimum 150GB free space** on your system drive:
  - 100GB for Windows 10 VM disk
  - 5-10GB for Windows 10 ISO download
  - 20-40GB for Windows installation + drivers
  - 10GB+ per VR game

### **Hardware Requirements**
- **NVIDIA GPU**: Any NVIDIA GPU (GeForce, Quadro, or Tesla)
  - GTX 600 series or newer
  - RTX series recommended for best VR performance
  - Minimum 4GB VRAM (6GB+ recommended for VR)
  - **Works with ANY driver state**: nvidia, nouveau, or no driver
- **CPU**: Must support virtualization (VT-x for Intel, AMD-V for AMD)
  - 6+ cores recommended (4 for VM, 2+ for host)
- **RAM**: Minimum 16GB (8GB for VM, 8GB for host)
  - 24GB+ recommended for best performance
- **Motherboard**: IOMMU/VT-d support (most modern boards)
- **VR Headset**: Any SteamVR or OpenXR compatible headset

### **Software Requirements**
- **Linux Distribution**: 
  - ✅ **Nobara Linux** (Recommended - optimized for gaming)
  - ✅ **Fedora** (Fully supported)
  - ✅ **Other Fedora-based**: RHEL, CentOS Stream, etc.
  - ⚠️ **Ubuntu/Debian**: Requires manual package installation (see below)
  - ⚠️ **Arch/Manjaro**: Requires manual package installation (see below)
- **Rust**: For compiling Ten Pod (will guide installation if missing)

### **Supported VR Headsets**
- ✅ Valve Index
- ✅ Meta Quest 2/3/Pro (wired or Link cable)
- ✅ HTC Vive / Vive Pro / Vive Cosmos
- ✅ HP Reverb G2
- ✅ Pico 4
- ✅ Pimax 5K/8K
- ✅ Any other SteamVR headset

---

## ✨ Key Features

- **Universal NVIDIA GPU Support**: Works with ANY NVIDIA GPU and driver state
  - ✅ Proprietary nvidia driver installed
  - ✅ Open-source nouveau driver installed
  - ✅ No driver installed at all
  - Ten Pod automatically handles driver unbinding during installation
- **Automatic Headset Detection**: Plug and play with any SteamVR headset
- **Zero Performance Loss**: Direct GPU passthrough (0-5% overhead vs native)
- **One-Click Setup**: Automated installation with guided steps
- **Safe and Reversible**: Easy to uninstall and return GPU to Linux

---

## 🚀 Quick Start

### Step 1: Download Ten Pod

```bash
# Clone or download the repository
cd ~
mkdir tenpod && cd tenpod

# Download tenpod.rs (paste the code from this repo)
# OR
wget https://raw.githubusercontent.com/YOUR_REPO/tenpod/main/tenpod.rs

# Make it executable
chmod +x tenpod.rs
```

### Step 2: Install Rust (if not already installed)

```bash
# Check if Rust is installed
rustc --version

# If not installed, install Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 3: Compile Ten Pod

```bash
rustc tenpod.rs -o tenpod
```

### Step 4: Check System Status

```bash
./tenpod status
```

This will show you:
- ✅ GPU detection
- ✅ Headset detection
- ✅ System requirements check

### Step 5: Enable IOMMU in BIOS

**BEFORE running the installer**, ensure IOMMU is enabled:

1. Reboot and enter BIOS/UEFI settings (usually Del, F2, or F12 during boot)
2. Enable these settings:
   - **Intel**: VT-x and VT-d
   - **AMD**: AMD-V and IOMMU
3. Save and exit BIOS

### Step 6: Configure GRUB for IOMMU

```bash
# For AMD CPUs:
sudo tee /etc/default/grub << 'EOF'
GRUB_TIMEOUT=5
GRUB_DISTRIBUTOR="$(sed 's, release .*$,,g' /etc/system-release)"
GRUB_DEFAULT=saved
GRUB_DISABLE_SUBMENU=true
GRUB_TERMINAL_OUTPUT="console"
GRUB_CMDLINE_LINUX="rhgb quiet amd_iommu=on iommu=pt"
GRUB_DISABLE_RECOVERY="true"
GRUB_ENABLE_BLSCFG=true
EOF

# For Intel CPUs:
sudo tee /etc/default/grub << 'EOF'
GRUB_TIMEOUT=5
GRUB_DISTRIBUTOR="$(sed 's, release .*$,,g' /etc/system-release)"
GRUB_DEFAULT=saved
GRUB_DISABLE_SUBMENU=true
GRUB_TERMINAL_OUTPUT="console"
GRUB_CMDLINE_LINUX="rhgb quiet intel_iommu=on iommu=pt"
GRUB_DISABLE_RECOVERY="true"
GRUB_ENABLE_BLSCFG=true
EOF

# Rebuild GRUB and reboot
sudo grub2-mkconfig -o /boot/grub2/grub.cfg
sudo reboot
```

### Step 7: Run the Installer

```bash
cd ~/tenpod
./tenpod install
```

**The installer will:**
- Install QEMU/KVM packages (using `dnf` for Fedora/Nobara)
- Configure VFIO for GPU passthrough
- Set up hugepages for performance
- Add your user to required groups
- Create a 100GB VM disk

**When prompted, type `y` to continue.**

⚠️ **REBOOT REQUIRED** after installation completes!

```bash
sudo reboot
```

---

## 🐧 Other Linux Distributions

### **Ubuntu/Debian/Pop!_OS**

The current Ten Pod installer uses `dnf` (Fedora's package manager). For Ubuntu-based systems, you'll need to manually install packages first:

```bash
# Install required packages
sudo apt update
sudo apt install -y qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils virt-manager ovmf

# Enable libvirt
sudo systemctl enable --now libvirtd

# Add user to groups
sudo usermod -aG libvirt,kvm $USER

# Then run Ten Pod installer (it will skip package installation)
cd ~/tenpod
./tenpod install
```

### **Arch Linux/Manjaro**

```bash
# Install required packages
sudo pacman -S qemu libvirt virt-manager bridge-utils dnsmasq ovmf

# Enable libvirt
sudo systemctl enable --now libvirtd

# Add user to groups
sudo usermod -aG libvirt,kvm $USER

# Then run Ten Pod installer
cd ~/tenpod
./tenpod install
```

**Note:** The GRUB configuration steps remain the same for all distributions.

---

## 📀 Step 8: Download Windows 10 ISO

### **CRITICAL: Windows 10 ISO Requirements**

You **MUST** download the **64-bit** version of Windows 10. Do NOT use 32-bit.

### **Official Download (FREE)**

**Direct Link:** [Download Windows 10 ISO (Microsoft Official)](https://www.microsoft.com/software-download/windows10)

1. Click the link above or go to: https://www.microsoft.com/software-download/windows10
2. Click **"Download tool now"** OR **"Download Windows 10 Disc Image (ISO)"**
   - **On Linux**: The page will detect Linux and offer direct ISO download
   - **On Windows**: You'll need to use the Media Creation Tool
3. Select:
   - **Language**: English (or your preference)
   - **Edition**: Windows 10 64-bit (x64) ← **IMPORTANT: Must be 64-bit!**
4. Download will be **~5-6GB** (make sure you have space!)

**💡 Pro Tip:** If you're downloading from Linux (which you are!), Microsoft's website will automatically show you the direct ISO download option without needing the Media Creation Tool.

### **Move ISO to Correct Location**

```bash
# After download completes (assuming it's in ~/Downloads)
sudo mv ~/Downloads/Win10*.iso /var/lib/libvirt/images/win10.iso

# Verify it's there
ls -lh /var/lib/libvirt/images/win10.iso
```

### **Alternative: Direct Download (if you have a link)**

```bash
cd /var/lib/libvirt/images/
sudo wget -O win10.iso "YOUR_DIRECT_DOWNLOAD_LINK"
```

---

## 🎮 Step 9: Launch Ten Pod

### **Before Starting:**
1. ✅ Ensure Windows 10 ISO is at `/var/lib/libvirt/images/win10.iso`
2. ✅ **Plug in your VR headset** (USB cable connected)
3. ✅ Make sure you have at least **150GB free space**

### **Start the VM:**

```bash
cd ~/tenpod
./tenpod start
```

**What happens:**
- Windows 10 will boot from the ISO (first-time setup)
- Your RTX GPU is passed directly to Windows
- Your VR headset is passed to Windows
- You'll see Windows installation screen on your monitor

### **Inside Windows (First Time):**

1. **Install Windows 10**
   - Follow the installation wizard
   - Skip product key (click "I don't have a product key")
   - Choose "Windows 10 Home" or "Pro"
   - Select "Custom Install"
   - Install to the virtual disk

2. **Install NVIDIA Drivers**
   - **Direct Link:** [NVIDIA Driver Downloads](https://www.nvidia.com/Download/index.aspx)
   - Or go to: https://www.nvidia.com/Download/index.aspx
   - Select your GPU model (e.g., RTX 2060)
   - Download and install drivers
   - Reboot Windows after installation

3. **Install Steam**
   - **Direct Link:** [Download Steam](https://store.steampowered.com/about/)
   - Or go to: https://store.steampowered.com/about/
   - Install Steam
   - Log in to your account
   - Install SteamVR from your Steam Library

4. **Install VR Games**
   - Beat Saber, Half-Life: Alyx, Boneworks, etc.
   - Launch games through SteamVR
   - Put on your headset and enjoy!

---

## 🎯 Daily Usage

### **Starting VR Gaming Session:**

```bash
cd ~/tenpod

# Plug in VR headset first!

# Start the VM
./tenpod start

# Wait for Windows to boot
# Put on headset and play!
```

### **Stopping the VM:**

```bash
# Properly shutdown Windows from inside the VM first (Start → Shutdown)
# OR force stop:
./tenpod stop
```

### **Check Status:**

```bash
./tenpod status
```

---

## 🔧 Troubleshooting

### **IOMMU Not Enabled**

```bash
# Check if IOMMU is working
sudo dmesg | grep -i iommu

# Should see output like:
# AMD-Vi: IOMMU enabled
# OR
# Intel-VT-d: IOMMU enabled
```

**If not working:**
1. Check BIOS settings (VT-d/IOMMU enabled)
2. Verify GRUB config: `cat /proc/cmdline`
3. Should contain: `amd_iommu=on` or `intel_iommu=on`

### **GPU Not Detected**

```bash
lspci | grep NVIDIA
```

Should show your GPU. If not, check:
- GPU is properly seated
- Power cables connected
- Try different PCIe slot

### **"GPU is using nvidia/nouveau driver" Warning**

**This is normal!** Ten Pod will automatically unbind the driver during installation. 

The installer creates VFIO configuration that takes priority over nvidia/nouveau drivers. After reboot following installation, your GPU will automatically bind to vfio-pci instead.

**You DO NOT need to:**
- ❌ Manually uninstall nvidia drivers
- ❌ Manually uninstall nouveau drivers
- ❌ Blacklist drivers yourself (Ten Pod does this)

**The process:**
1. Install Ten Pod → Creates VFIO config
2. Reboot → GPU automatically binds to vfio-pci
3. Start VM → GPU is ready for passthrough

**To verify it worked after reboot:**
```bash
# Check which driver your GPU is using
lspci -nnk | grep -A3 NVIDIA

# Should show:
# Kernel driver in use: vfio-pci
```

If it still shows nvidia or nouveau after reboot, run:
```bash
sudo dracut -f --force
sudo reboot
```

### **VM Won't Start**

```bash
# Check if disk exists
ls -lh /var/lib/libvirt/images/win10_tenpod.qcow2

# Check if ISO exists
ls -lh /var/lib/libvirt/images/win10.iso

# Check permissions
sudo chown -R $USER:$USER /var/lib/libvirt/images/
```

### **Headset Not Detected in Windows**

1. Unplug and replug headset USB
2. Stop VM with `./tenpod stop`
3. Plug in headset
4. Start VM with `./tenpod start`
5. Check Windows Device Manager for headset

### **Poor VR Performance**

1. **Check CPU pinning**: Ensure cores 0-3 are free for host
   ```bash
   htop  # Check CPU usage
   ```

2. **Verify hugepages**:
   ```bash
   cat /proc/meminfo | grep Huge
   ```

3. **Inside Windows**:
   - Update NVIDIA drivers
   - Close background apps
   - Set Windows Power Plan to "High Performance"
   - Disable Windows Update during gaming

---

## 🗑️ Uninstallation

### **Remove VFIO (Return GPU to Linux)**

```bash
# Remove VFIO configuration
sudo rm /etc/modprobe.d/vfio.conf
sudo rm /etc/modprobe.d/vfio-priority.conf

# Rebuild initramfs
sudo dracut -f --force

# Reboot
sudo reboot
```

**After reboot:**
- Your GPU will automatically bind to nvidia or nouveau driver
- Linux desktop will use the GPU normally
- You can install nvidia drivers if you want: `sudo dnf install akmod-nvidia`

### **Remove VM Disk (Free up space)**

```bash
# WARNING: This deletes your Windows installation!
sudo rm /var/lib/libvirt/images/win10_tenpod.qcow2
sudo rm /var/lib/libvirt/images/win10.iso
```

---

## 📊 Performance Tips

### **Optimal Settings:**

1. **RAM Allocation**: 
   - 16GB total? Use 8GB for VM
   - 24GB total? Use 12GB for VM
   - 32GB total? Use 16GB for VM

2. **CPU Cores**:
   - 8-core CPU: Give 4-6 cores to VM
   - 12-core CPU: Give 6-8 cores to VM
   - Leave at least 2 cores for Linux host

3. **Inside Windows**:
   - Disable Windows Defender real-time scanning
   - Disable background apps
   - Set SteamVR to 90Hz or 120Hz depending on GPU

---

## 🆘 Getting Help

**If you encounter issues:**

1. Run diagnostic:
   ```bash
   ./tenpod status
   sudo dmesg | grep -i iommu
   lspci -nnk | grep -A3 NVIDIA
   ```

2. Check logs:
   ```bash
   journalctl -xe | grep qemu
   ```

3. Post issue on GitHub with:
   - Output of `./tenpod status`
   - Your GPU model
   - Your VR headset model
   - Error messages

---

## ⚖️ License

MIT License - Free to use, modify, and distribute.

---

## 🙏 Credits

Built with:
- QEMU/KVM - Virtual machine
- VFIO - GPU passthrough
- Rust - System programming
- Linux - Freedom

**Ten Pod** - Making VR gaming on Linux actually work.

**Compatible with:**
- ✅ Nobara Linux (Recommended)
- ✅ Fedora / RHEL / CentOS Stream
- ✅ Ubuntu / Debian / Pop!_OS (manual package install required)
- ✅ Arch Linux / Manjaro (manual package install required)
- Any NVIDIA GPU (proprietary driver, nouveau, or no driver)
- Any SteamVR or OpenXR compatible VR headset

---

## ⚠️ Final Warnings

- **This modifies your system configuration** (GRUB, kernel modules)
- **Backup important data** before installation
- **You need 150GB+ free space** for the full setup
- **IOMMU must be enabled** in BIOS
- **Reboot is required** after installation
- **Windows 10 license is optional** (works without activation)
- **GPU will be unavailable to Linux** when VM is running

**By using this software, you accept these requirements and risks.**

---

## 🚀 Ready to Game?

```bash
cd ~/tenpod
./tenpod status    # Check everything
./tenpod install   # One-time setup
sudo reboot        # Required!
./tenpod start     # Launch and play!
```

**Happy VR gaming!** 🎮🥽

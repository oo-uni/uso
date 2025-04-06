### **1️⃣ Core Components**  
🟢 **USB Hub IC** → Allows multiple USB devices  
   - **FE1.1S** (Basic 4-port USB 2.0 hub)  
   - **GL850G** (Reliable 4-port hub, low power)  
   - **VL817** (USB 3.0, if we want faster data)  

🟢 **USB Audio & MIDI Processing**  
   - **Teensy 4.1** (Can handle USB audio & MIDI)  
   - **RP2040 (Pi Pico)** (Cheaper, but needs a USB bridge)  
   - **Custom Linux board (CM4, RPi Zero 2)** (More powerful, but overkill?)  

🟢 **ADC/DAC for Audio Conversion**  
   - **CS4272 / PCM2902** (Pro audio quality)  
   - **WM8731** (Used in many pro devices)  

🟢 **USB-C Power Handling**  
   - **PD-compatible circuit** for handling different power sources  

---

## **2️⃣ Features & Design**  
✅ **USB Hub with 4+ Ports** (for MIDI controllers, synths, etc.)  
✅ **2 XLR/TRS Mic Inputs** (low-noise preamps)  
✅ **Class-Compliant Audio & MIDI** (no drivers needed)  
✅ **Bus-Powered OR External Power Option**  
✅ **Expandable Firmware** (future updates)  

---

## **🔥 Next Steps?**  
1️⃣ **Prototype with an off-the-shelf USB hub + Teensy**  
2️⃣ Test **multiple MIDI devices** + **low-latency audio**  
3️⃣ Design a **custom PCB** with a USB hub + audio interface  
4️⃣ Optimize for the mobile app  

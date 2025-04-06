Alright, let’s define the **core software functionalities** for UNIBOX.  

### **Phase 1: Core Functionalities to Build**
#### ✅ **1. MIDI & Audio Interface Management**  
- Detect & map **connected MIDI controllers** (via USB & possible Bluetooth).  
- Handle **MIDI routing** (controller → software → external hardware).  
- Process **audio I/O** (low-latency handling of input/output).  
- Built-in **mixer with gain, EQ, FX send/return**.  

#### ✅ **2. Unified Control Interface**  
- Modular UI that adapts to **touchscreen** and **MIDI hardware**.  
- User-defined **MIDI mappings** (knobs, faders, pads, etc.).  
- Gesture-based interaction (future EMS/gesture control compatibility).  

#### ✅ **3. Performance Features**  
- **Looping & Layering** (real-time loop recording + overdubs).  
- **AI-assisted automation** (dynamic volume, tempo sync, live effects).  
- **Multi-track recording** (with auto-sync for post-production).  

#### ✅ **4. Cross-Platform Compatibility**  
- Runs on **Android, Linux, and Windows**.  
- **USB Host & Client Mode** (can be a MIDI host or act as a controller for another device).  

---

### **Phase 1 Development Approach**  
💡 **Step 1: Audio & MIDI Framework**  
→ Choose a cross-platform audio engine (JUCE, PortAudio, RtAudio, or ALSA+JACK for Linux).  
→ Implement a MIDI manager to detect & route MIDI signals.  

💡 **Step 2: UI Prototyping**  
→ Design a flexible UI for **touch & hardware MIDI controls**.  
→ Build a basic virtual mixer for handling inputs/outputs.  

💡 **Step 3: Real-time Performance Features**  
→ Develop a simple loop recorder.  
→ Implement automation (basic AI-driven FX or tempo sync).  

---

### **Questions Before We Start Coding:**  
1️⃣ **Audio Engine Choice**: Should we use **JUCE** (cross-platform, robust) or something more lightweight like **RtAudio/PortAudio**?  
2️⃣ **MIDI Handling**: Do we build a custom MIDI routing system or use an existing library?  
3️⃣ **UI Framework**: Should we go with **Qt (C++)**, **React (JS/TS for web-based UI)**, or something like **TouchDesigner** for rapid prototyping?  

Would you like to start with a **basic proof-of-concept UI** first or go straight into **MIDI + audio engine implementation**?
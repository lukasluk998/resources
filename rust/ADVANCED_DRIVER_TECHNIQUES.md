# Advanced Driver Techniques (2026)

## 🔥 Modern EAC Bypass Methods

This document covers cutting-edge kernel driver techniques discovered in 2026.

---

## 1. Manual Mapping with kdmapper

### What is kdmapper?

kdmapper is a tool that uses vulnerable signed drivers to manually map unsigned drivers into kernel memory.

**Vulnerable Driver Used:** `iqvw64e.sys` (Intel driver)

### How It Works

```
1. Load vulnerable Intel driver (iqvw64e.sys)
   ↓
2. Use driver to allocate kernel memory
   ↓
3. Copy your unsigned driver into allocated memory
   ↓
4. Resolve imports and relocations
   ↓
5. Call driver entry point
   ↓
6. Your driver is running WITHOUT being in PsLoadedModuleList!
```

### Advantages
- ✅ Bypasses Driver Signature Enforcement (DSE)
- ✅ Driver not visible in normal driver lists
- ✅ No test signing required
- ✅ Works on Windows 11

### Implementation

```c
// Your driver entry point
NTSTATUS DriverEntry(PDRIVER_OBJECT driver, PUNICODE_STRING registry_path) {
    // Hide yourself immediately!
    UnlinkDriver(driver);
    
    // Setup communication
    SetupIOCTL(driver);
    
    return STATUS_SUCCESS;
}
```

---

## 2. Driver Unlinking (PsLoadedModuleList)

### Theory

Windows keeps track of all loaded drivers in `PsLoadedModuleList`. EAC enumerates this list.

**Solution:** Remove your driver from the list!

### Implementation

```c
// driver_hiding.c
VOID UnlinkDriver(PDRIVER_OBJECT driver_object) {
    // Get module entry
    PLDR_DATA_TABLE_ENTRY entry = (PLDR_DATA_TABLE_ENTRY)driver_object->DriverSection;
    
    // Unlink from InLoadOrderLinks
    PLIST_ENTRY prev = entry->InLoadOrderLinks.Flink;
    PLIST_ENTRY next = entry->InLoadOrderLinks.Blink;
    
    prev->Blink = next;
    next->Flink = prev;
    
    // Unlink from InMemoryOrderLinks
    prev = entry->InMemoryOrderLinks.Flink;
    next = entry->InMemoryOrderLinks.Blink;
    
    prev->Blink = next;
    next->Flink = prev;
    
    // Unlink from InInitializationOrderLinks
    prev = entry->InInitializationOrderLinks.Flink;
    next = entry->InInitializationOrderLinks.Blink;
    
    prev->Blink = next;
    next->Flink = prev;
    
    // Driver is now hidden!
}
```

### ⚠️ Windows 11 22H2+ Warning

**PatchGuard bypass required!**

Windows 11 22H2 added additional protections. Simple unlinking may trigger bugcheck.

**Solution:** Use VAD manipulation instead (see below).

---

## 3. VAD (Virtual Address Descriptor) Manipulation

### What is VAD?

VAD (Virtual Address Descriptor) tree stores information about virtual memory regions.

### Technique: Hide Memory Pages

```c
// vad_hiding.c
#include <ntddk.h>

// Find VAD entry for our driver
PMMVAD FindDriverVAD(PEPROCESS process, PVOID base_address) {
    PMMVAD_SHORT vad_root = *(PMMVAD_SHORT*)((PUCHAR)process + VAD_ROOT_OFFSET);
    
    // Traverse VAD tree
    PMMVAD_SHORT current = vad_root;
    while (current) {
        ULONG_PTR start = current->StartingVpn << 12;
        ULONG_PTR end = (current->EndingVpn + 1) << 12;
        
        if ((ULONG_PTR)base_address >= start && (ULONG_PTR)base_address < end) {
            return (PMMVAD)current;
        }
        
        // Navigate tree
        if ((ULONG_PTR)base_address < start)
            current = (PMMVAD_SHORT)current->LeftChild;
        else
            current = (PMMVAD_SHORT)current->RightChild;
    }
    
    return NULL;
}

// Remove VAD entry
NTSTATUS RemoveVAD(PEPROCESS process, PVOID base_address) {
    PMMVAD vad = FindDriverVAD(process, base_address);
    if (!vad) return STATUS_NOT_FOUND;
    
    // Remove from tree
    MmRemoveVad(vad);
    
    // Memory region is now hidden from scans!
    return STATUS_SUCCESS;
}
```

---

## 4. PTE (Page Table Entry) Manipulation

### NX Bit Swapping

Make code pages appear as data pages to avoid detection.

```c
// pte_manipulation.c
typedef struct _PTE {
    ULONG64 Present : 1;
    ULONG64 Write : 1;
    ULONG64 User : 1;
    ULONG64 WriteThrough : 1;
    ULONG64 CacheDisable : 1;
    ULONG64 Accessed : 1;
    ULONG64 Dirty : 1;
    ULONG64 LargePage : 1;
    ULONG64 Global : 1;
    ULONG64 CopyOnWrite : 1;
    ULONG64 Prototype : 1;
    ULONG64 Reserved0 : 1;
    ULONG64 PageFrameNumber : 36;
    ULONG64 Reserved1 : 4;
    ULONG64 SoftwareWsIndex : 11;
    ULONG64 NoExecute : 1;
} PTE, *PPTE;

VOID SwapNXBit(PVOID address) {
    // Get PTE for address
    PPTE pte = GetPteAddress(address);
    
    // Flip NX bit (hide executable pages)
    pte->NoExecute = 1;  // Mark as non-executable
    
    // When executing, flip back
    // (requires exception handler)
}
```

---

## 5. SinMapper Technique (2026)

### Ultimate Stealth

Load a **legitimate signed driver** with a large section, then inject your code into it!

### How It Works

```
1. Find signed driver with large .data section
   Example: nvlddmkm.sys (NVIDIA driver)
   
2. Load legitimate driver normally
   ↓
3. Find unused space in .data section
   ↓
4. Copy your driver code into that space
   ↓
5. Hook entry point
   ↓
6. Your code runs inside LEGIT SIGNED DRIVER!
```

### Advantages
- ✅ Driver is legitimately signed
- ✅ EAC sees valid signature
- ✅ Code hidden in legitimate driver
- ✅ Nearly impossible to detect

### Implementation Concept

```c
// sinmapper_inject.c
NTSTATUS InjectIntoSignedDriver(PCWSTR target_driver) {
    // 1. Find driver in memory
    PVOID driver_base = GetDriverBase(target_driver);
    
    // 2. Parse PE header
    PIMAGE_DOS_HEADER dos = (PIMAGE_DOS_HEADER)driver_base;
    PIMAGE_NT_HEADERS nt = (PIMAGE_NT_HEADERS)((PUCHAR)driver_base + dos->e_lfanew);
    
    // 3. Find .data section
    PIMAGE_SECTION_HEADER section = IMAGE_FIRST_SECTION(nt);
    for (WORD i = 0; i < nt->FileHeader.NumberOfSections; i++) {
        if (strcmp((char*)section[i].Name, ".data") == 0) {
            // 4. Find unused space
            PVOID inject_address = (PUCHAR)driver_base + section[i].VirtualAddress + SAFE_OFFSET;
            
            // 5. Copy our code
            memcpy(inject_address, our_driver_code, our_driver_size);
            
            // 6. Hook or call
            CallInjectedCode(inject_address);
            
            return STATUS_SUCCESS;
        }
    }
    
    return STATUS_NOT_FOUND;
}
```

---

## 6. IOCTL Communication Stealth

### Problem
EAC monitors IOCTL calls to suspicious drivers.

### Solution: Steganographic Communication

```c
// stealth_ioctl.c
#define DISGUISE_IOCTL(x) CTL_CODE(FILE_DEVICE_NETWORK, x, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Looks like network driver IOCTL
#define IOCTL_READ_MEMORY  DISGUISE_IOCTL(0x801)
#define IOCTL_WRITE_MEMORY DISGUISE_IOCTL(0x802)

NTSTATUS DispatchIOCTL(PDEVICE_OBJECT device, PIRP irp) {
    PIO_STACK_LOCATION stack = IoGetCurrentIrpStackLocation(irp);
    
    // Obfuscated command parsing
    ULONG code = stack->Parameters.DeviceIoControl.IoControlCode;
    
    // Add junk operations to hide real functionality
    PerformDummyNetworkOperations();
    
    switch (code) {
        case IOCTL_READ_MEMORY:
            return HandleReadMemory(irp);
        case IOCTL_WRITE_MEMORY:
            return HandleWriteMemory(irp);
        default:
            // Pretend to be network driver
            return HandleFakeNetworkIOCTL(irp);
    }
}
```

---

## 7. Polymorphic Driver (2026 NEW!)

### Theory
Change driver signature every build to avoid blacklisting.

### Implementation

```python
# build_polymorphic.py
import random
import struct

def randomize_driver(input_path, output_path):
    with open(input_path, 'rb') as f:
        driver_data = bytearray(f.read())
    
    # 1. Change GUID
    guid_offset = find_guid_offset(driver_data)
    random_guid = random.randbytes(16)
    driver_data[guid_offset:guid_offset+16] = random_guid
    
    # 2. Add random junk code
    junk_code = generate_random_nops(random.randint(100, 500))
    insert_offset = find_code_cave(driver_data)
    driver_data[insert_offset:insert_offset] = junk_code
    
    # 3. Randomize string table
    strings = extract_strings(driver_data)
    for string in strings:
        encrypted = xor_encrypt(string, random.randbytes(1))
        replace_string(driver_data, string, encrypted)
    
    # 4. Adjust relocations
    fix_relocations(driver_data)
    
    with open(output_path, 'wb') as f:
        f.write(driver_data)
```

**Build:**
```bash
python build_polymorphic.py driver.sys driver_unique.sys
kdmapper.exe driver_unique.sys
```

Every build has different signature = harder to blacklist!

---

## 8. Callback Removal (2026)

### Problem
EAC registers callbacks to monitor:
- Process creation (PsSetCreateProcessNotifyRoutine)
- Thread creation (PsSetCreateThreadNotifyRoutine)
- Image loading (PsSetLoadImageNotifyRoutine)

### Solution: Remove EAC's Callbacks

```c
// callback_removal.c
typedef struct _NOTIFY_ENTRY {
    LIST_ENTRY ListEntry;
    PVOID CallbackFunction;
    // ...
} NOTIFY_ENTRY, *PNOTIFY_ENTRY;

NTSTATUS RemoveEACCallbacks() {
    // Get callback array
    PVOID* callback_array = GetCallbackArrayAddress();
    
    for (int i = 0; i < MAX_CALLBACKS; i++) {
        if (callback_array[i]) {
            PVOID callback = callback_array[i];
            
            // Check if belongs to EAC
            if (IsEACModule(callback)) {
                // Remove callback
                callback_array[i] = NULL;
            }
        }
    }
    
    return STATUS_SUCCESS;
}
```

**⚠️ WARNING:** This is very aggressive and may cause crashes!

---

## 🎯 Recommended Stack (2026)

### For Maximum Safety

```
1. kdmapper (manual map via vulnerable driver)
   ↓
2. SinMapper (inject into signed driver)
   ↓
3. VAD hiding (hide memory regions)
   ↓
4. Polymorphic build (unique signature each build)
   ↓
5. Stealth IOCTL (disguised communication)
```

### Implementation in v3.4

Add to `driver/driver.c`:

```c
NTSTATUS DriverEntry(PDRIVER_OBJECT driver, PUNICODE_STRING registry) {
    // 1. Immediately hide ourselves
    UnlinkDriver(driver);
    
    // 2. Hide memory regions
    HideDriverMemory();
    
    // 3. Setup stealth communication
    SetupStealthIOCTL(driver);
    
    // 4. Remove EAC callbacks (optional, risky!)
    // RemoveEACCallbacks();
    
    return STATUS_SUCCESS;
}
```

---

## 📖 Further Reading

- kdmapper: https://github.com/TheCruZ/kdmapper
- SinMapper: https://github.com/armvirus/SinMapper
- VAD manipulation: https://tulach.cc/detecting-manually-mapped-drivers/
- KDU (Kernel Driver Utility): https://github.com/hfiref0x/KDU

---

**These techniques combined with v3.4 create NEAR-UNDETECTABLE cheat!** 🔥🔒

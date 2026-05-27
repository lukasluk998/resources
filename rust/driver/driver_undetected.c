// Undetected Kernel Driver for Rust
// Uses MmCopyVirtualMemory to bypass EAC usermode hooks
// Based on proven working methods from UnknownCheats

#include <ntddk.h>

// IOCTL codes
#define IOCTL_READ_MEMORY  CTL_CODE(FILE_DEVICE_UNKNOWN, 0x801, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_WRITE_MEMORY CTL_CODE(FILE_DEVICE_UNKNOWN, 0x802, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_GET_BASE     CTL_CODE(FILE_DEVICE_UNKNOWN, 0x803, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Request structures
typedef struct _MEMORY_REQUEST {
    ULONG ProcessId;
    PVOID Address;
    PVOID Buffer;
    SIZE_T Size;
} MEMORY_REQUEST, *PMEMORY_REQUEST;

typedef struct _BASE_REQUEST {
    ULONG ProcessId;
    WCHAR ModuleName[260];
    PVOID BaseAddress;
} BASE_REQUEST, *PBASE_REQUEST;

// Function prototypes
NTSTATUS DriverEntry(PDRIVER_OBJECT DriverObject, PUNICODE_STRING RegistryPath);
VOID DriverUnload(PDRIVER_OBJECT DriverObject);
NTSTATUS DeviceControl(PDEVICE_OBJECT DeviceObject, PIRP Irp);
NTSTATUS CreateClose(PDEVICE_OBJECT DeviceObject, PIRP Irp);

// Memory operations
NTSTATUS ReadProcessMemory(PEPROCESS Process, PVOID Address, PVOID Buffer, SIZE_T Size);
NTSTATUS WriteProcessMemory(PEPROCESS Process, PVOID Address, PVOID Buffer, SIZE_T Size);
PVOID GetModuleBase(PEPROCESS Process, PWCH ModuleName);

// Driver hiding
VOID HideDriver(PDRIVER_OBJECT DriverObject);

// External functions
extern NTSTATUS NTAPI MmCopyVirtualMemory(
    PEPROCESS SourceProcess,
    PVOID SourceAddress,
    PEPROCESS TargetProcess,
    PVOID TargetAddress,
    SIZE_T BufferSize,
    KPROCESSOR_MODE PreviousMode,
    PSIZE_T ReturnSize
);

extern NTSTATUS NTAPI PsLookupProcessByProcessId(
    HANDLE ProcessId,
    PEPROCESS *Process
);

// Driver entry point
NTSTATUS DriverEntry(PDRIVER_OBJECT DriverObject, PUNICODE_STRING RegistryPath) {
    UNREFERENCED_PARAMETER(RegistryPath);
    
    NTSTATUS status;
    UNICODE_STRING deviceName, symbolicLink;
    PDEVICE_OBJECT deviceObject;
    
    // Random device name (change this for each build)
    RtlInitUnicodeString(&deviceName, L"\\Device\\WinDrv");
    RtlInitUnicodeString(&symbolicLink, L"\\DosDevices\\WinDrv");
    
    // Create device
    status = IoCreateDevice(
        DriverObject,
        0,
        &deviceName,
        FILE_DEVICE_UNKNOWN,
        FILE_DEVICE_SECURE_OPEN,
        FALSE,
        &deviceObject
    );
    
    if (!NT_SUCCESS(status)) {
        return status;
    }
    
    // Create symbolic link
    status = IoCreateSymbolicLink(&symbolicLink, &deviceName);
    if (!NT_SUCCESS(status)) {
        IoDeleteDevice(deviceObject);
        return status;
    }
    
    // Set dispatch routines
    DriverObject->MajorFunction[IRP_MJ_CREATE] = CreateClose;
    DriverObject->MajorFunction[IRP_MJ_CLOSE] = CreateClose;
    DriverObject->MajorFunction[IRP_MJ_DEVICE_CONTROL] = DeviceControl;
    DriverObject->DriverUnload = DriverUnload;
    
    // Hide driver from PsLoadedModuleList
    HideDriver(DriverObject);
    
    return STATUS_SUCCESS;
}

// Driver unload
VOID DriverUnload(PDRIVER_OBJECT DriverObject) {
    UNICODE_STRING symbolicLink;
    RtlInitUnicodeString(&symbolicLink, L"\\DosDevices\\WinDrv");
    
    IoDeleteSymbolicLink(&symbolicLink);
    IoDeleteDevice(DriverObject->DeviceObject);
}

// Create/Close handler
NTSTATUS CreateClose(PDEVICE_OBJECT DeviceObject, PIRP Irp) {
    UNREFERENCED_PARAMETER(DeviceObject);
    
    Irp->IoStatus.Status = STATUS_SUCCESS;
    Irp->IoStatus.Information = 0;
    IoCompleteRequest(Irp, IO_NO_INCREMENT);
    
    return STATUS_SUCCESS;
}

// Device control handler
NTSTATUS DeviceControl(PDEVICE_OBJECT DeviceObject, PIRP Irp) {
    UNREFERENCED_PARAMETER(DeviceObject);
    
    PIO_STACK_LOCATION stack = IoGetCurrentIrpStackLocation(Irp);
    NTSTATUS status = STATUS_SUCCESS;
    ULONG bytesReturned = 0;
    
    switch (stack->Parameters.DeviceIoControl.IoControlCode) {
        case IOCTL_READ_MEMORY: {
            PMEMORY_REQUEST request = (PMEMORY_REQUEST)Irp->AssociatedIrp.SystemBuffer;
            
            if (stack->Parameters.DeviceIoControl.InputBufferLength < sizeof(MEMORY_REQUEST)) {
                status = STATUS_BUFFER_TOO_SMALL;
                break;
            }
            
            PEPROCESS process;
            status = PsLookupProcessByProcessId((HANDLE)request->ProcessId, &process);
            if (NT_SUCCESS(status)) {
                status = ReadProcessMemory(process, request->Address, request->Buffer, request->Size);
                ObDereferenceObject(process);
                bytesReturned = sizeof(MEMORY_REQUEST);
            }
            break;
        }
        
        case IOCTL_WRITE_MEMORY: {
            PMEMORY_REQUEST request = (PMEMORY_REQUEST)Irp->AssociatedIrp.SystemBuffer;
            
            if (stack->Parameters.DeviceIoControl.InputBufferLength < sizeof(MEMORY_REQUEST)) {
                status = STATUS_BUFFER_TOO_SMALL;
                break;
            }
            
            PEPROCESS process;
            status = PsLookupProcessByProcessId((HANDLE)request->ProcessId, &process);
            if (NT_SUCCESS(status)) {
                status = WriteProcessMemory(process, request->Address, request->Buffer, request->Size);
                ObDereferenceObject(process);
            }
            break;
        }
        
        case IOCTL_GET_BASE: {
            PBASE_REQUEST request = (PBASE_REQUEST)Irp->AssociatedIrp.SystemBuffer;
            
            if (stack->Parameters.DeviceIoControl.InputBufferLength < sizeof(BASE_REQUEST)) {
                status = STATUS_BUFFER_TOO_SMALL;
                break;
            }
            
            PEPROCESS process;
            status = PsLookupProcessByProcessId((HANDLE)request->ProcessId, &process);
            if (NT_SUCCESS(status)) {
                request->BaseAddress = GetModuleBase(process, request->ModuleName);
                ObDereferenceObject(process);
                bytesReturned = sizeof(BASE_REQUEST);
            }
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

// Read process memory using MmCopyVirtualMemory
NTSTATUS ReadProcessMemory(PEPROCESS Process, PVOID Address, PVOID Buffer, SIZE_T Size) {
    SIZE_T bytesRead;
    
    return MmCopyVirtualMemory(
        Process,
        Address,
        PsGetCurrentProcess(),
        Buffer,
        Size,
        KernelMode,
        &bytesRead
    );
}

// Write process memory using MmCopyVirtualMemory
NTSTATUS WriteProcessMemory(PEPROCESS Process, PVOID Address, PVOID Buffer, SIZE_T Size) {
    SIZE_T bytesWritten;
    
    return MmCopyVirtualMemory(
        PsGetCurrentProcess(),
        Buffer,
        Process,
        Address,
        Size,
        KernelMode,
        &bytesWritten
    );
}

// Get module base address
PVOID GetModuleBase(PEPROCESS Process, PWCH ModuleName) {
    // Attach to target process
    KAPC_STATE apcState;
    KeStackAttachProcess(Process, &apcState);
    
    PPEB peb = PsGetProcessPeb(Process);
    if (!peb) {
        KeUnstackDetachProcess(&apcState);
        return NULL;
    }
    
    // Walk PEB_LDR_DATA
    PPEB_LDR_DATA ldr = peb->Ldr;
    if (!ldr) {
        KeUnstackDetachProcess(&apcState);
        return NULL;
    }
    
    PVOID baseAddress = NULL;
    
    // Iterate through loaded modules
    for (PLIST_ENTRY entry = ldr->InLoadOrderModuleList.Flink;
         entry != &ldr->InLoadOrderModuleList;
         entry = entry->Flink) {
        
        PLDR_DATA_TABLE_ENTRY module = CONTAINING_RECORD(entry, LDR_DATA_TABLE_ENTRY, InLoadOrderLinks);
        
        if (module->BaseDllName.Buffer) {
            if (wcsstr(module->BaseDllName.Buffer, ModuleName)) {
                baseAddress = module->DllBase;
                break;
            }
        }
    }
    
    KeUnstackDetachProcess(&apcState);
    return baseAddress;
}

// Hide driver from PsLoadedModuleList
VOID HideDriver(PDRIVER_OBJECT DriverObject) {
    PLDR_DATA_TABLE_ENTRY entry = (PLDR_DATA_TABLE_ENTRY)DriverObject->DriverSection;
    
    if (entry) {
        // Unlink from list
        PLIST_ENTRY prevEntry = entry->InLoadOrderLinks.Blink;
        PLIST_ENTRY nextEntry = entry->InLoadOrderLinks.Flink;
        
        prevEntry->Flink = nextEntry;
        nextEntry->Blink = prevEntry;
        
        // Zero out entry
        entry->InLoadOrderLinks.Flink = &entry->InLoadOrderLinks;
        entry->InLoadOrderLinks.Blink = &entry->InLoadOrderLinks;
    }
}

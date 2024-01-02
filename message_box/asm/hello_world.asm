section .text
    global WinMainCRTStartup

    extern MessageBoxA, ExitProcess, GetModuleHandle

    section .data
    MB_OK equ 0
    MB_ICONINFORMATION equ 0x40
    NULL equ 0

section .text
WinMainCRTStartup:
    ; Get the handle to the current module (executable)
    push NULL
    call GetModuleHandle
    mov ebx, eax ; Save the module handle

    ; Display a message box with the module handle as the caption
    push MB_OK | MB_ICONINFORMATION ; uType
    push ebx ; lpText
    push ebx ; lpCaption
    push NULL ; hWnd
    call MessageBoxA

    ; Exit the process
    push 0 ; uExitCode
    call ExitProcess


; Flat Assembler (FASM) x64 shellcode template
; Microsoft x86 stdcall calling convention
; 'return_buffer' is injected into code at runtime.
; Position independent logic.
;
; Pseudo code:
;
; loadlibraryw(path)
; {
;   var result = load_library_w(path);
;   *return_buffer = result;
;
;   if (result == NULL)
;     return get_last_error();
;
;   return SUCCESS;
; }


use32

loadlibraryw:
    mov eax, [esp+4] ; Grab CreateRemoteThread lpParameter from stack
    push eax         ; Push lpParameter
    mov eax, 0x11223344 ; Call LoadLibraryW
    call eax
    mov dword [dword 0x11223344], eax ; Store result in return_buffer

    xor  eax, eax  ; Set return value to 0 (success)
    jne   finish

getlasterror:
    mov  eax, 0x11223344
    call eax

finish:
    ret 4      ; Return, assuming 8 bytes of arguments 
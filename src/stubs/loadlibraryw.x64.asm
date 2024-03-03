; Flat Assembler (FASM) x64 shellcode template
; Microsoft x64 calling convention
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


use64
sub rsp, 40 ; Re-align stack to 16 byte boundary + shadow space.

loadlibraryw:
    mov rax, 0x1122334455667788  ; Placeholder for 'load_library_w' address
    call rax                     ; Call LoadLibraryW
    mov qword [qword 0x1122334455667788], rax  ; Placeholder for 'return_buffer'

    xor rax, rax ; Set 'success', and flag.
    jne finish

getlasterror:
    mov rax, 0x1122334455667788 ; Placeholder for 'get_last_error' address
    call rax

finish:
    add rsp, 40 ; Re-align stack to 16 byte boundary + shadow space.
    ret ; return in rax
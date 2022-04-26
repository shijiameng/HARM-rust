#ifndef SECURE_VENEER_H
#define SECURE_VENEER_H

#define NONSECURE_ENTRY_ASM __attribute__((cmse_nonsecure_entry, naked))
#define NONSECURE_ENTRY __attribute__((cmse_nonsecure_entry))

#ifdef __cplusplus
extern "C" {
#endif 

NONSECURE_ENTRY_ASM void secure_fn_return(void);

NONSECURE_ENTRY_ASM void secure_indirect_call(void);

#ifdef __cplusplus
}
#endif 

#endif
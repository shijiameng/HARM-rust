#include <stdint.h>

#include "tz_veneer.h"
#include "dispatch_tbl.h"
#include "callsite_tbl.h"

__attribute__((used)) static const uint32_t __dispatch_tbl_sz = DISPATCH_TABLE_SIZE;
__attribute__((used)) static const uint32_t __callsite_tbl_sz = CALLSITE_TABLE_SIZE;


__attribute__((used, noreturn)) static void __panic(void)
{
    __asm volatile("cpsid i");
    for (;;);
}

void secure_fn_return(void)
{
    __asm volatile(
        "  .syntax unified                \n"
        "  .extern CALLSITE_TBL           \n"
        "  .extern DISPATCH_TBL           \n"
        "  .extern __panic                \n"
        "                                 \n"
        "  push   {r0-r2}                 \n"
        "  ubfx   r0, lr, #1, #27         \n"
        "  ldr    r1, =__callsite_tbl_sz  \n"
        "  ldr    r1, [r1]                \n"
        "  cmp    r0, r1                  \n"
        "  bge    __panic                 \n"
        "  ldr    r1, =CALLSITE_TBL       \n"
        "  ldr    r1, [r1, r0, lsl #2]    \n"
        "  uxth   r0, r1                  \n"
        "  lsr    r1, r1, #16             \n"
        "  ldr    r2, =DISPATCH_TBL       \n"
        "  ldr    r2, [r2, r1, lsl #2]    \n"
        "  add    lr, r2, r0              \n"
        "  pop    {r0-r2}                 \n"
        "  bxns   lr                      \n"
    );
}


void secure_indirect_call(void)
{
    __asm volatile(
        "  .syntax unified                \n"
        "  .extern DISPATCH_TBL           \n"
        "  .extern __dispatch_tbl_sz      \n"
        "  .extern __panic                \n"
        "                                 \n"
        "  push   {r0-r1}                 \n"
        "  mov    r0, %0                  \n"
        "  and    r1, r12, r0             \n"
        "  cmp    r1, r0                  \n"
        "  bne    __panic                 \n"
        "  ubfx   r0, r12, #16, %1        \n"
        "  ldr    r1, =__dispatch_tbl_sz  \n"
        "  ldr    r1, [r1]                \n"
        "  cmp    r0, r1                  \n"
        "  bge    __panic                 \n"
        "  ldr    r1, =DISPATCH_TBL       \n"
        "  ldr    r1, [r1, r0, lsl #2]    \n"
        "  mov    r12, r1                 \n"
        "  pop    {r0-r1}                 \n"
        "  bxns   r12                     \n"
        :: "i"(DISPATCH_MAGIC), "i"(DISPATCH_INDEX_BITS)
    );
}
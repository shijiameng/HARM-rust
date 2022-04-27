/*
 * mpu_config.h
 *
 *  Created on: Sep 24, 2020
 *      Author: jiameng
 */

#ifndef MPU_CONFIG_H_
#define MPU_CONFIG_H_

#include <stdint.h>

/* Memory regions */
#define STACK_HEAP_BASE_S	0x30010000UL
#define STACK_HEAP_LIMIT_S	0x30015FFFUL

#define STACK_HEAP_BASE_NS	0x20000000UL
#define STACK_HEAP_LIMIT_NS	0x2000FFFFUL

#define SANDBOX_BASE		0x2001a000
#define SANDBOX_LIMIT		(0x2001a000 + 0x2a000)
#define SANDBOX_SIZE		(SANDBOX_LIMIT - SANDBOX_BASE)

#define PROGRAM_FLASH_BASE_S		0x10000000UL
#define PROGRAM_FLASH_LIMIT_S		0x1001FFFFUL

#define PROGRAM_FLASH_BASE_NS		0x00020000UL
#define PROGRAM_FLASH_LIMIT_NS		0x0008FFFFUL

/* General MPU Masks */
#define REGION_ENABLE		0x00000001

/* Shareability */
#define NON_SHAREABLE		0x00
#define RESERVED			0x08
#define OUTER_SHAREABLE		0x10
#define INNER_SHAREABLE		0x18

/* Access Permissions */
#define EXEC_NEVER			0x01	/* All instruction fetches abort */
#define RW_P_ONLY			0x00	/* Read/Write, Privileged code only */
#define RW_P_U				0x02	/* Read/Write, Any Privilege Level */
#define RO_P_ONLY			0x04	/* Read-Only, Privileged code only */
#define RO_P_U				0x06	/* Read-Only, Any Privilege Level */

/* Read/Write Allocation Configurations for Cacheable Memory
 * Attr<n>[7:4] and Attr<n>[3:0] are of the format: 0bXXRW
 */
#define R_NON_W_NON			0x0		/* Do not allocate Read/Write */
#define R_NON_W_ALLOC		0x1		/* Do not allocate Read, Allocate Write */
#define R_ALLOC_W_NON		0x2		/* Allocate Read, Do not allocate Write */
#define R_ALLOC_W_ALLOC		0x3		/* ALlocate Read/Write */

/* Memory Attribute Masks */
#define DEVICE				0x0F
#define NORMAL_OUTER		0xF0
#define NORMAL_INNER		0x0F

/* Memory Attributes */
#define DEVICE_NG_NR_NE		0x00	/* Device, Non-Gathering, Non-Reordering, Non-Early-Write-Acknowledgement */
#define DEVICE_NG_NR_E		0x04	/* Device, Non-Gathering, Non-Reordering, Early-Write_Acknowledgement */
#define DEVICE_NG_R_E   	0x08 	/* Device, Non-Gathering, Reordering, Early-Write-Acknowledgement */
#define DEVICE_G_R_E   		0x0C 	/* Device, Gathering, Reordering, Early-Write-Acknowledgement */

#define NORMAL_O_WT_T   	0x00 	/* Normal, Outer Write-through transient (if RW not 00) */
#define NORMAL_O_NC     	0x40 	/* Normal, Outer Non-cacheable (if RW is 00) */
#define NORMAL_O_WB_T   	0x40 	/* Normal, Outer Write-back transient (if RW not 00) */
#define NORMAL_O_WT_NT  	0x80 	/* Normal, Outer Write-through non-transient */
#define NORMAL_O_WB_NT  	0xC0 	/* Normal, Outer Write-back non-transient */

#define NORMAL_I_WT_T   	0x00 	/* Normal, Inner Write-through transient (if RW not 00) */
#define NORMAL_I_NC     	0x04 	/* Normal, Inner Non-cacheable (if RW is 00) */
#define NORMAL_I_WB_T   	0x04 	/* Normal, Inner Write-back transient (if RW not 00) */
#define NORMAL_I_WT_NT  	0x08 	/* Normal, Inner Write-through non-transient */
#define NORMAL_I_WB_NT  	0x0C 	/* Normal, Inner Write-back non-transient */

#ifdef __cplusplus
extern "C" {
#endif

void BOARD_InitMPU(void);

#ifdef __cplusplus
}
#endif

#endif /* MPU_CONFIG_H_ */

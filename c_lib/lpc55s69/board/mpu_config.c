/*
 * mpu_config.c
 *
 *  Created on: Oct 17, 2020
 *      Author: jiameng
 */


#include "mpu_config.h"
#include "LPC55S69_cm33_core0.h"

void BOARD_InitMPU(void)
{
	uint32_t non_cacheable_attr = NORMAL_O_NC | NORMAL_I_NC;
	uint32_t device_attr = DEVICE_NG_NR_NE;
	uint32_t mpu_regions, i;

	__DMB();

	MPU_NS->CTRL = 0;

	mpu_regions = (MPU_NS->TYPE & MPU_TYPE_DREGION_Msk) >> MPU_TYPE_DREGION_Pos;

	MPU_NS->MAIR0 |= ((device_attr << MPU_MAIR0_Attr1_Pos) | (non_cacheable_attr));

	/* Non-secure flash */
	MPU_NS->RNR = 0;
	MPU_NS->RBAR = (PROGRAM_FLASH_BASE_NS & MPU_RBAR_BASE_Msk) | NON_SHAREABLE | RO_P_U | EXEC_NEVER;
	MPU_NS->RLAR = (PROGRAM_FLASH_LIMIT_NS & MPU_RLAR_LIMIT_Msk) | ((0 << MPU_RLAR_AttrIndx_Pos) & MPU_RLAR_AttrIndx_Msk) | REGION_ENABLE;

	/* Non-secure stack and heap */
	MPU_NS->RNR = 1;
	MPU_NS->RBAR = (STACK_HEAP_BASE_NS & MPU_RBAR_BASE_Msk) | NON_SHAREABLE | RW_P_U | EXEC_NEVER;
	MPU_NS->RLAR = (STACK_HEAP_LIMIT_NS & MPU_RLAR_LIMIT_Msk) | ((0 << MPU_RLAR_AttrIndx_Pos) & MPU_RLAR_AttrIndx_Msk) | REGION_ENABLE;

	MPU_NS->RNR = 2;
	MPU_NS->RBAR = (SANDBOX_BASE & MPU_RBAR_BASE_Msk) | NON_SHAREABLE | RO_P_U;
	MPU_NS->RLAR = (SANDBOX_LIMIT & MPU_RLAR_LIMIT_Msk) | ((0 << MPU_RLAR_AttrIndx_Pos) & MPU_RLAR_AttrIndx_Msk) | REGION_ENABLE;

	for (i = 3; i < mpu_regions; i++) {
		MPU_NS->RNR = i;
		MPU_NS->RLAR &= 0;
	}

	MPU_NS->CTRL = 5;

	__DSB();
	__ISB();

	MPU->CTRL = 0;

	mpu_regions = (MPU->TYPE & MPU_TYPE_DREGION_Msk) >> MPU_TYPE_DREGION_Pos;

	/* Secure stack and heap */
	MPU->MAIR0 |= ((device_attr << MPU_MAIR0_Attr1_Pos) | (non_cacheable_attr));

	MPU->RNR = 0;
	MPU->RBAR = (STACK_HEAP_BASE_S & MPU_RBAR_BASE_Msk) | NON_SHAREABLE | RW_P_U | EXEC_NEVER;
	MPU->RLAR = (STACK_HEAP_LIMIT_S & MPU_RLAR_LIMIT_Msk) | ((0 << MPU_RLAR_AttrIndx_Pos) & MPU_RLAR_AttrIndx_Msk) | REGION_ENABLE;

	MPU->RNR = 1;
	MPU->RBAR = (PROGRAM_FLASH_BASE_S & MPU_RBAR_BASE_Msk) | NON_SHAREABLE | RO_P_U;
	MPU->RLAR = (PROGRAM_FLASH_LIMIT_S & MPU_RLAR_LIMIT_Msk) | ((0 << MPU_RLAR_AttrIndx_Pos) & MPU_RLAR_AttrIndx_Msk) | REGION_ENABLE;

	MPU->RNR = 2;
	MPU->RBAR = (0x50000000 & MPU_RBAR_BASE_Msk) | OUTER_SHAREABLE | RW_P_U;
	MPU->RLAR = (0x50103FFF & MPU_RLAR_LIMIT_Msk) | ((1 << MPU_RLAR_AttrIndx_Pos) & MPU_RLAR_AttrIndx_Msk) | REGION_ENABLE;

	for (i = 3; i < mpu_regions; i++) {
		MPU->RNR = i;
		MPU->RLAR &= 0x0;
	}

	MPU->CTRL = 5;

	__DSB();
	__ISB();
}


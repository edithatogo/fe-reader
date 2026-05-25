#ifndef FE_READER_C_ABI_H
#define FE_READER_C_ABI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define FE_READER_C_ABI_STATUS_OK 0
#define FE_READER_C_ABI_STATUS_UNSUPPORTED 1
#define FE_READER_C_ABI_STATUS_MUTATION_NOT_EXPOSED 2

typedef struct FeReaderCAbiPlanContract {
  uint32_t abi_version_major;
  uint32_t risk_level;
  uint32_t write_mode;
  uint32_t approved_for_apply;
  uint32_t operation_count;
  int32_t status;
} FeReaderCAbiPlanContract;

uint32_t fe_reader_c_abi_version_major(void);
uint32_t fe_reader_c_abi_version_minor(void);
uint32_t fe_reader_c_abi_version_patch(void);
const char *fe_reader_c_abi_contract_json(void);
uint32_t fe_reader_c_abi_supports_apply(void);
uint32_t fe_reader_c_abi_supports_plan_only(void);
FeReaderCAbiPlanContract fe_reader_c_abi_plan_noop_contract(void);

#ifdef __cplusplus
}
#endif

#endif

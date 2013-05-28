#[link(name = "opencl", vers = "0.1", uuid = "f83bfc2b-e3ee-4e4c-b324-70e379fbcff2")];

#[license = "MIT"];
#[crate_type = "lib"];

#[author = "Jens Nockert"];

#[comment = "OpenCL bindings for Rust"];
#[desc = "Bindings for OpenCL 1.2, mapping one-to-one to the C bindings"];

extern mod std;

/* FreeBSD, Windows and Android missing */

#[nolink]
#[link_args = "-framework OpenCL"]
#[cfg(target_os = "macos")]
extern { }

#[nolink]
#[link_args = "-lOpenCL"]
#[cfg(target_os = "linux")]
extern { }

macro_rules! cl_type(($name:ident, $t:ty) => (pub type $name = $t;))
macro_rules! cl_constant(($t:ty, $name:ident = $v:expr) => (pub static $name:$t = $v;))

macro_rules! cl_error(($name:ident = $v:expr) => (cl_constant!(cl_int, $name = $v))) //(pub static $name: cl_int = $v;))

/*
 * OpenCL 1.0 cl_platform.h
 */

/* scalar types */
cl_type!(cl_char, i8)
cl_type!(cl_uchar, u8)
cl_type!(cl_short, i16)
cl_type!(cl_ushort, u16)
cl_type!(cl_int, i32)
cl_type!(cl_uint, u32)
cl_type!(cl_long, i64)
cl_type!(cl_ulong, u64)

cl_type!(cl_half, u16)
cl_type!(cl_float, f32)
cl_type!(cl_double, f64)

/* TODO: Lots of macros */
/* TODO: Vector types */
/* TODO: Alignment */
/* TODO: GL types */
    
/*
 * OpenCL 1.0 cl.h
 */
    
cl_type!(cl_platform_id, *libc::c_void)
cl_type!(cl_device_id, *libc::c_void)
cl_type!(cl_context, *libc::c_void)
cl_type!(cl_command_queue, *libc::c_void)
cl_type!(cl_mem, *libc::c_void)
cl_type!(cl_program, *libc::c_void)
cl_type!(cl_kernel, *libc::c_void)
cl_type!(cl_event, *libc::c_void)
cl_type!(cl_sampler, *libc::c_void)

cl_type!(cl_bool, cl_uint)
cl_type!(cl_bitfield, cl_ulong)
cl_type!(cl_device_type, cl_bitfield)
cl_type!(cl_platform_info, cl_uint)
cl_type!(cl_device_info, cl_uint)
/* cl_type!(cl_device_address_info, cl_bitfield) was removed from 1.1 and not used in the 1.0 API */
cl_type!(cl_device_fp_config, cl_bitfield)
cl_type!(cl_device_mem_cache_type, cl_uint)
cl_type!(cl_device_local_mem_type, cl_uint)
cl_type!(cl_device_exec_capabilities, cl_bitfield)
cl_type!(cl_command_queue_properties, cl_bitfield)

cl_type!(cl_context_properties, libc::intptr_t)
cl_type!(cl_context_info, cl_uint)
cl_type!(cl_command_queue_info, cl_uint)
cl_type!(cl_channel_order, cl_uint)
cl_type!(cl_channel_type, cl_uint)
cl_type!(cl_mem_flags, cl_bitfield)
cl_type!(cl_mem_object_type, cl_uint)
cl_type!(cl_mem_info, cl_uint)
cl_type!(cl_image_info, cl_uint)
cl_type!(cl_buffer_create_type, cl_uint)
cl_type!(cl_addressing_mode, cl_uint)
cl_type!(cl_filter_mode, cl_uint)
cl_type!(cl_sampler_info, cl_uint)
cl_type!(cl_map_flags, cl_bitfield)
cl_type!(cl_program_info, cl_uint)
cl_type!(cl_program_build_info, cl_uint)
cl_type!(cl_build_status, cl_int)
cl_type!(cl_kernel_info, cl_uint)
cl_type!(cl_kernel_work_group_info, cl_uint)
cl_type!(cl_event_info, cl_uint)
cl_type!(cl_command_type, cl_uint)
cl_type!(cl_profiling_info, cl_uint)

pub struct cl_image_format {
    image_channel_order: cl_channel_order,
    image_channel_data_type: cl_channel_type
}

/* Error Codes */
cl_error!(CL_SUCCESS = 0)
cl_error!(CL_DEVICE_NOT_FOUND = -1)
cl_error!(CL_DEVICE_NOT_AVAILABLE = -2)
cl_error!(CL_COMPILER_NOT_AVAILABLE = -3)
cl_error!(CL_MEM_OBJECT_ALLOCATION_FAILURE = -4)
cl_error!(CL_OUT_OF_RESOURCES = -5)
cl_error!(CL_OUT_OF_HOST_MEMORY = -6)
cl_error!(CL_PROFILING_INFO_NOT_AVAILABLE = -7)
cl_error!(CL_MEM_COPY_OVERLAP = -8)
cl_error!(CL_IMAGE_FORMAT_MISMATCH = -9)
cl_error!(CL_IMAGE_FORMAT_NOT_SUPPORTED = -10)
cl_error!(CL_BUILD_PROGRAM_FAILURE = -11)
cl_error!(CL_MAP_FAILURE = -12)

cl_error!(CL_INVALID_VALUE = -30)
cl_error!(CL_INVALID_DEVICE_TYPE = -31)
cl_error!(CL_INVALID_PLATFORM = -32)
cl_error!(CL_INVALID_DEVICE = -33)
cl_error!(CL_INVALID_CONTEXT = -34)
cl_error!(CL_INVALID_QUEUE_PROPERTIES = -35)
cl_error!(CL_INVALID_COMMAND_QUEUE = -36)
cl_error!(CL_INVALID_HOST_PTR = -37)
cl_error!(CL_INVALID_MEM_OBJECT = -38)
cl_error!(CL_INVALID_IMAGE_FORMAT_DESCRIPTOR = -39)
cl_error!(CL_INVALID_IMAGE_SIZE = -40)
cl_error!(CL_INVALID_SAMPLER = -41)
cl_error!(CL_INVALID_BINARY = -42)
cl_error!(CL_INVALID_BUILD_OPTIONS = -43)
cl_error!(CL_INVALID_PROGRAM = -44)
cl_error!(CL_INVALID_PROGRAM_EXECUTABLE = -45)
cl_error!(CL_INVALID_KERNEL_NAME = -46)
cl_error!(CL_INVALID_KERNEL_DEFINITION = -47)
cl_error!(CL_INVALID_KERNEL = -48)
cl_error!(CL_INVALID_ARG_INDEX = -49)
cl_error!(CL_INVALID_ARG_VALUE = -50)
cl_error!(CL_INVALID_ARG_SIZE = -51)
cl_error!(CL_INVALID_KERNEL_ARGS = -52)
cl_error!(CL_INVALID_WORK_DIMENSION = -53)
cl_error!(CL_INVALID_WORK_GROUP_SIZE = -54)
cl_error!(CL_INVALID_WORK_ITEM_SIZE = -55)
cl_error!(CL_INVALID_GLOBAL_OFFSET = -56)
cl_error!(CL_INVALID_EVENT_WAIT_LIST = -57)
cl_error!(CL_INVALID_EVENT = -58)
cl_error!(CL_INVALID_OPERATION = -59)
cl_error!(CL_INVALID_GL_OBJECT = -60)
cl_error!(CL_INVALID_BUFFER_SIZE = -61)
cl_error!(CL_INVALID_MIP_LEVEL = -62)
cl_error!(CL_INVALID_GLOBAL_WORK_SIZE = -63)

/* OpenCL Version */
cl_constant!(cl_int, CL_VERSION_1_0 = 1)

/* cl_bool */
cl_constant!(cl_bool, CL_FALSE = 0)
cl_constant!(cl_bool, CL_TRUE = 1)

/* cl_platform_info */
cl_constant!(cl_platform_info, CL_PLATFORM_PROFILE = 0x0900)
cl_constant!(cl_platform_info, CL_PLATFORM_VERSION = 0x0901)
cl_constant!(cl_platform_info, CL_PLATFORM_NAME = 0x0902)
cl_constant!(cl_platform_info, CL_PLATFORM_VENDOR = 0x0903)
cl_constant!(cl_platform_info, CL_PLATFORM_EXTENSIONS = 0x0904)

/* cl_device_type - bitfield */
cl_constant!(cl_device_type, CL_DEVICE_TYPE_DEFAULT = (1 << 0))
cl_constant!(cl_device_type, CL_DEVICE_TYPE_CPU = (1 << 1))
cl_constant!(cl_device_type, CL_DEVICE_TYPE_GPU = (1 << 2))
cl_constant!(cl_device_type, CL_DEVICE_TYPE_ACCELERATOR = (1 << 3))
cl_constant!(cl_device_type, CL_DEVICE_TYPE_ALL = 0xFFFFFFFF)

/* cl_device_info */
cl_constant!(cl_device_info, CL_DEVICE_TYPE = 0x1000)
cl_constant!(cl_device_info, CL_DEVICE_VENDOR_ID = 0x1001)
cl_constant!(cl_device_info, CL_DEVICE_MAX_COMPUTE_UNITS = 0x1002)
cl_constant!(cl_device_info, CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS = 0x1003)
cl_constant!(cl_device_info, CL_DEVICE_MAX_WORK_GROUP_SIZE = 0x1004)
cl_constant!(cl_device_info, CL_DEVICE_MAX_WORK_ITEM_SIZES = 0x1005)
cl_constant!(cl_device_info, CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR = 0x1006)
cl_constant!(cl_device_info, CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT = 0x1007)
cl_constant!(cl_device_info, CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT = 0x1008)
cl_constant!(cl_device_info, CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG = 0x1009)
cl_constant!(cl_device_info, CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT = 0x100A)
cl_constant!(cl_device_info, CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE = 0x100B)
cl_constant!(cl_device_info, CL_DEVICE_MAX_CLOCK_FREQUENCY = 0x100C)
cl_constant!(cl_device_info, CL_DEVICE_ADDRESS_BITS = 0x100D)
cl_constant!(cl_device_info, CL_DEVICE_MAX_READ_IMAGE_ARGS = 0x100E)
cl_constant!(cl_device_info, CL_DEVICE_MAX_WRITE_IMAGE_ARGS = 0x100F)
cl_constant!(cl_device_info, CL_DEVICE_MAX_MEM_ALLOC_SIZE = 0x1010)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE2D_MAX_WIDTH = 0x1011)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE2D_MAX_HEIGHT = 0x1012)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE3D_MAX_WIDTH = 0x1013)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE3D_MAX_HEIGHT = 0x1014)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE3D_MAX_DEPTH = 0x1015)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE_SUPPORT = 0x1016)
cl_constant!(cl_device_info, CL_DEVICE_MAX_PARAMETER_SIZE = 0x1017)
cl_constant!(cl_device_info, CL_DEVICE_MAX_SAMPLERS = 0x1018)
cl_constant!(cl_device_info, CL_DEVICE_MEM_BASE_ADDR_ALIGN = 0x1019)
cl_constant!(cl_device_info, CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE = 0x101A)
cl_constant!(cl_device_info, CL_DEVICE_SINGLE_FP_CONFIG = 0x101B)
cl_constant!(cl_device_info, CL_DEVICE_GLOBAL_MEM_CACHE_TYPE = 0x101C)
cl_constant!(cl_device_info, CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE = 0x101D)
cl_constant!(cl_device_info, CL_DEVICE_GLOBAL_MEM_CACHE_SIZE = 0x101E)
cl_constant!(cl_device_info, CL_DEVICE_GLOBAL_MEM_SIZE = 0x101F)
cl_constant!(cl_device_info, CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE = 0x1020)
cl_constant!(cl_device_info, CL_DEVICE_MAX_CONSTANT_ARGS = 0x1021)
cl_constant!(cl_device_info, CL_DEVICE_LOCAL_MEM_TYPE = 0x1022)
cl_constant!(cl_device_info, CL_DEVICE_LOCAL_MEM_SIZE = 0x1023)
cl_constant!(cl_device_info, CL_DEVICE_ERROR_CORRECTION_SUPPORT = 0x1024)
cl_constant!(cl_device_info, CL_DEVICE_PROFILING_TIMER_RESOLUTION = 0x1025)
cl_constant!(cl_device_info, CL_DEVICE_ENDIAN_LITTLE = 0x1026)
cl_constant!(cl_device_info, CL_DEVICE_AVAILABLE = 0x1027)
cl_constant!(cl_device_info, CL_DEVICE_COMPILER_AVAILABLE = 0x1028)
cl_constant!(cl_device_info, CL_DEVICE_EXECUTION_CAPABILITIES = 0x1029)
cl_constant!(cl_device_info, CL_DEVICE_QUEUE_PROPERTIES = 0x102A)
cl_constant!(cl_device_info, CL_DEVICE_NAME = 0x102B)
cl_constant!(cl_device_info, CL_DEVICE_VENDOR = 0x102C)
cl_constant!(cl_device_info, CL_DRIVER_VERSION = 0x102D)
cl_constant!(cl_device_info, CL_DEVICE_PROFILE = 0x102E)
cl_constant!(cl_device_info, CL_DEVICE_VERSION = 0x102F)
cl_constant!(cl_device_info, CL_DEVICE_EXTENSIONS = 0x1030)
cl_constant!(cl_device_info, CL_DEVICE_PLATFORM = 0x1031)

/* 0x1032 reserved for CL_DEVICE_DOUBLE_FP_CONFIG */
/* 0x1033 reserved for CL_DEVICE_HALF_FP_CONFIG */

/* cl_device_fp_config - bitfield */
cl_constant!(cl_device_fp_config, CL_FP_DENORM = (1 << 0))
cl_constant!(cl_device_fp_config, CL_FP_INF_NAN = (1 << 1))
cl_constant!(cl_device_fp_config, CL_FP_ROUND_TO_NEAREST = (1 << 2))
cl_constant!(cl_device_fp_config, CL_FP_ROUND_TO_ZERO = (1 << 3))
cl_constant!(cl_device_fp_config, CL_FP_ROUND_TO_INF = (1 << 4))
cl_constant!(cl_device_fp_config, CL_FP_FMA = (1 << 5))

/* cl_device_mem_cache_type */
cl_constant!(cl_device_mem_cache_type, CL_NONE = 0x0)
cl_constant!(cl_device_mem_cache_type, CL_READ_ONLY_CACHE = 0x1)
cl_constant!(cl_device_mem_cache_type, CL_READ_WRITE_CACHE = 0x2)

/* cl_device_local_mem_type */
cl_constant!(cl_device_local_mem_type, CL_LOCAL = 0x1)
cl_constant!(cl_device_local_mem_type, CL_GLOBAL = 0x2)

/* cl_device_exec_capabilities - bitfield */
cl_constant!(cl_device_exec_capabilities, CL_EXEC_KERNEL = (1 << 0))
cl_constant!(cl_device_exec_capabilities, CL_EXEC_NATIVE_KERNEL = (1 << 1))

/* cl_command_queue_properties - bitfield */
cl_constant!(cl_command_queue_properties, CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE = (1 << 0))
cl_constant!(cl_command_queue_properties, CL_QUEUE_PROFILING_ENABLE = (1 << 1))

/* cl_context_info  */
cl_constant!(cl_context_info, CL_CONTEXT_REFERENCE_COUNT = 0x1080)
cl_constant!(cl_context_info, CL_CONTEXT_DEVICES = 0x1081)
cl_constant!(cl_context_info, CL_CONTEXT_PROPERTIES = 0x1082)

/* cl_context_info + cl_context_properties */
cl_constant!(cl_int, CL_CONTEXT_PLATFORM = 0x1084)

/* cl_command_queue_info */
cl_constant!(cl_command_queue_info, CL_QUEUE_CONTEXT = 0x1090)
cl_constant!(cl_command_queue_info, CL_QUEUE_DEVICE = 0x1091)
cl_constant!(cl_command_queue_info, CL_QUEUE_REFERENCE_COUNT = 0x1092)
cl_constant!(cl_command_queue_info, CL_QUEUE_PROPERTIES = 0x1093)

/* cl_mem_flags - bitfield */
cl_constant!(cl_mem_flags, CL_MEM_READ_WRITE = (1 << 0))
cl_constant!(cl_mem_flags, CL_MEM_WRITE_ONLY = (1 << 1))
cl_constant!(cl_mem_flags, CL_MEM_READ_ONLY = (1 << 2))
cl_constant!(cl_mem_flags, CL_MEM_USE_HOST_PTR = (1 << 3))
cl_constant!(cl_mem_flags, CL_MEM_ALLOC_HOST_PTR = (1 << 4))
cl_constant!(cl_mem_flags, CL_MEM_COPY_HOST_PTR = (1 << 5))

/* cl_channel_order */
cl_constant!(cl_channel_order, CL_R = 0x10B0)
cl_constant!(cl_channel_order, CL_A = 0x10B1)
cl_constant!(cl_channel_order, CL_RG = 0x10B2)
cl_constant!(cl_channel_order, CL_RA = 0x10B3)
cl_constant!(cl_channel_order, CL_RGB = 0x10B4)
cl_constant!(cl_channel_order, CL_RGBA = 0x10B5)
cl_constant!(cl_channel_order, CL_BGRA = 0x10B6)
cl_constant!(cl_channel_order, CL_ARGB = 0x10B7)
cl_constant!(cl_channel_order, CL_INTENSITY = 0x10B8)
cl_constant!(cl_channel_order, CL_LUMINANCE = 0x10B9)

/* cl_channel_type */
cl_constant!(cl_channel_type, CL_SNORM_INT8 = 0x10D0)
cl_constant!(cl_channel_type, CL_SNORM_INT16 = 0x10D1)
cl_constant!(cl_channel_type, CL_UNORM_INT8 = 0x10D2)
cl_constant!(cl_channel_type, CL_UNORM_INT16 = 0x10D3)
cl_constant!(cl_channel_type, CL_UNORM_SHORT_565 = 0x10D4)
cl_constant!(cl_channel_type, CL_UNORM_SHORT_555 = 0x10D5)
cl_constant!(cl_channel_type, CL_UNORM_INT_101010 = 0x10D6)
cl_constant!(cl_channel_type, CL_SIGNED_INT8 = 0x10D7)
cl_constant!(cl_channel_type, CL_SIGNED_INT16 = 0x10D8)
cl_constant!(cl_channel_type, CL_SIGNED_INT32 = 0x10D9)
cl_constant!(cl_channel_type, CL_UNSIGNED_INT8 = 0x10DA)
cl_constant!(cl_channel_type, CL_UNSIGNED_INT16 = 0x10DB)
cl_constant!(cl_channel_type, CL_UNSIGNED_INT32 = 0x10DC)
cl_constant!(cl_channel_type, CL_HALF_FLOAT = 0x10DD)
cl_constant!(cl_channel_type, CL_FLOAT = 0x10DE)

/* cl_mem_object_type */
cl_constant!(cl_mem_object_type, CL_MEM_OBJECT_BUFFER = 0x10F0)
cl_constant!(cl_mem_object_type, CL_MEM_OBJECT_IMAGE2D = 0x10F1)
cl_constant!(cl_mem_object_type, CL_MEM_OBJECT_IMAGE3D = 0x10F2)

/* cl_mem_info */
cl_constant!(cl_mem_info, CL_MEM_TYPE = 0x1100)
cl_constant!(cl_mem_info, CL_MEM_FLAGS = 0x1101)
cl_constant!(cl_mem_info, CL_MEM_SIZE = 0x1102)
cl_constant!(cl_mem_info, CL_MEM_HOST_PTR = 0x1103)
cl_constant!(cl_mem_info, CL_MEM_MAP_COUNT = 0x1104)
cl_constant!(cl_mem_info, CL_MEM_REFERENCE_COUNT = 0x1105)
cl_constant!(cl_mem_info, CL_MEM_CONTEXT = 0x1106)

/* cl_image_info */
cl_constant!(cl_image_info, CL_IMAGE_FORMAT = 0x1110)
cl_constant!(cl_image_info, CL_IMAGE_ELEMENT_SIZE = 0x1111)
cl_constant!(cl_image_info, CL_IMAGE_ROW_PITCH = 0x1112)
cl_constant!(cl_image_info, CL_IMAGE_SLICE_PITCH = 0x1113)
cl_constant!(cl_image_info, CL_IMAGE_WIDTH = 0x1114)
cl_constant!(cl_image_info, CL_IMAGE_HEIGHT = 0x1115)
cl_constant!(cl_image_info, CL_IMAGE_DEPTH = 0x1116)

/* cl_addressing_mode */
cl_constant!(cl_addressing_mode, CL_ADDRESS_NONE = 0x1130)
cl_constant!(cl_addressing_mode, CL_ADDRESS_CLAMP_TO_EDGE = 0x1131)
cl_constant!(cl_addressing_mode, CL_ADDRESS_CLAMP = 0x1132)
cl_constant!(cl_addressing_mode, CL_ADDRESS_REPEAT = 0x1133)

/* cl_filter_mode */
cl_constant!(cl_filter_mode, CL_FILTER_NEAREST = 0x1140)
cl_constant!(cl_filter_mode, CL_FILTER_LINEAR = 0x1141)

/* cl_sampler_info */
cl_constant!(cl_sampler_info, CL_SAMPLER_REFERENCE_COUNT = 0x1150)
cl_constant!(cl_sampler_info, CL_SAMPLER_CONTEXT = 0x1151)
cl_constant!(cl_sampler_info, CL_SAMPLER_NORMALIZED_COORDS = 0x1152)
cl_constant!(cl_sampler_info, CL_SAMPLER_ADDRESSING_MODE = 0x1153)
cl_constant!(cl_sampler_info, CL_SAMPLER_FILTER_MODE = 0x1154)

/* cl_map_flags - bitfield */
cl_constant!(cl_map_flags, CL_MAP_READ = (1 << 0))
cl_constant!(cl_map_flags, CL_MAP_WRITE = (1 << 1))

/* cl_program_info */
cl_constant!(cl_program_info, CL_PROGRAM_REFERENCE_COUNT = 0x1160)
cl_constant!(cl_program_info, CL_PROGRAM_CONTEXT = 0x1161)
cl_constant!(cl_program_info, CL_PROGRAM_NUM_DEVICES = 0x1162)
cl_constant!(cl_program_info, CL_PROGRAM_DEVICES = 0x1163)
cl_constant!(cl_program_info, CL_PROGRAM_SOURCE = 0x1164)
cl_constant!(cl_program_info, CL_PROGRAM_BINARY_SIZES = 0x1165)
cl_constant!(cl_program_info, CL_PROGRAM_BINARIES = 0x1166)

/* cl_program_build_info */
cl_constant!(cl_program_build_info, CL_PROGRAM_BUILD_STATUS = 0x1181)
cl_constant!(cl_program_build_info, CL_PROGRAM_BUILD_OPTIONS = 0x1182)
cl_constant!(cl_program_build_info, CL_PROGRAM_BUILD_LOG = 0x1183)

/* cl_build_status */
cl_constant!(cl_build_status, CL_BUILD_SUCCESS = 0)
cl_constant!(cl_build_status, CL_BUILD_NONE = -1)
cl_constant!(cl_build_status, CL_BUILD_ERROR = -2)
cl_constant!(cl_build_status, CL_BUILD_IN_PROGRESS = -3)

/* cl_kernel_info */
cl_constant!(cl_kernel_info, CL_KERNEL_FUNCTION_NAME = 0x1190)
cl_constant!(cl_kernel_info, CL_KERNEL_NUM_ARGS = 0x1191)
cl_constant!(cl_kernel_info, CL_KERNEL_REFERENCE_COUNT = 0x1192)
cl_constant!(cl_kernel_info, CL_KERNEL_CONTEXT = 0x1193)
cl_constant!(cl_kernel_info, CL_KERNEL_PROGRAM = 0x1194)

/* cl_kernel_work_group_info */
cl_constant!(cl_kernel_work_group_info, CL_KERNEL_WORK_GROUP_SIZE = 0x11B0)
cl_constant!(cl_kernel_work_group_info, CL_KERNEL_COMPILE_WORK_GROUP_SIZE = 0x11B1)
cl_constant!(cl_kernel_work_group_info, CL_KERNEL_LOCAL_MEM_SIZE = 0x11B2)

/* cl_event_info  */
cl_constant!(cl_event_info, CL_EVENT_COMMAND_QUEUE = 0x11D0)
cl_constant!(cl_event_info, CL_EVENT_COMMAND_TYPE = 0x11D1)
cl_constant!(cl_event_info, CL_EVENT_REFERENCE_COUNT = 0x11D2)
cl_constant!(cl_event_info, CL_EVENT_COMMAND_EXECUTION_STATUS = 0x11D3)

/* cl_command_type */
cl_constant!(cl_command_type, CL_COMMAND_NDRANGE_KERNEL = 0x11F0)
cl_constant!(cl_command_type, CL_COMMAND_TASK = 0x11F1)
cl_constant!(cl_command_type, CL_COMMAND_NATIVE_KERNEL = 0x11F2)
cl_constant!(cl_command_type, CL_COMMAND_READ_BUFFER = 0x11F3)
cl_constant!(cl_command_type, CL_COMMAND_WRITE_BUFFER = 0x11F4)
cl_constant!(cl_command_type, CL_COMMAND_COPY_BUFFER = 0x11F5)
cl_constant!(cl_command_type, CL_COMMAND_READ_IMAGE = 0x11F6)
cl_constant!(cl_command_type, CL_COMMAND_WRITE_IMAGE = 0x11F7)
cl_constant!(cl_command_type, CL_COMMAND_COPY_IMAGE = 0x11F8)
cl_constant!(cl_command_type, CL_COMMAND_COPY_IMAGE_TO_BUFFER = 0x11F9)
cl_constant!(cl_command_type, CL_COMMAND_COPY_BUFFER_TO_IMAGE = 0x11FA)
cl_constant!(cl_command_type, CL_COMMAND_MAP_BUFFER = 0x11FB)
cl_constant!(cl_command_type, CL_COMMAND_MAP_IMAGE = 0x11FC)
cl_constant!(cl_command_type, CL_COMMAND_UNMAP_MEM_OBJECT = 0x11FD)
cl_constant!(cl_command_type, CL_COMMAND_MARKER = 0x11FE)
cl_constant!(cl_command_type, CL_COMMAND_ACQUIRE_GL_OBJECTS = 0x11FF)
cl_constant!(cl_command_type, CL_COMMAND_RELEASE_GL_OBJECTS = 0x1200)

/* command execution status */
cl_constant!(cl_int, CL_COMPLETE = 0x0)
cl_constant!(cl_int, CL_RUNNING = 0x1)
cl_constant!(cl_int, CL_SUBMITTED = 0x2)
cl_constant!(cl_int, CL_QUEUED = 0x3)
  
/* cl_profiling_info  */
cl_constant!(cl_profiling_info, CL_PROFILING_COMMAND_QUEUED = 0x1280)
cl_constant!(cl_profiling_info, CL_PROFILING_COMMAND_SUBMIT = 0x1281)
cl_constant!(cl_profiling_info, CL_PROFILING_COMMAND_START = 0x1282)
cl_constant!(cl_profiling_info, CL_PROFILING_COMMAND_END = 0x1283)


#[nolink]
pub extern {
    /* Platform API */
    unsafe fn clGetPlatformIDs(num_entries: cl_uint,
                               platforms: *mut cl_platform_id,
                               num_platforms: *mut cl_uint) -> cl_int;
    unsafe fn clGetPlatformInfo(platform: cl_platform_id,
                                param_name: cl_platform_info,
                                param_value_size: libc::size_t,
                                param_value: *mut libc::c_void,
                                param_value_size_ret: *mut libc::size_t) -> cl_int;

    /* Device APIs */
    unsafe fn clGetDeviceIDs(platform: cl_platform_id,
                             device_type: cl_device_type,
                             num_entries: cl_uint,
                             devices: *mut cl_device_id,
                             num_devices: *mut cl_uint) -> cl_int;
    unsafe fn clGetDeviceInfo(device: cl_device_id,
                              param_name: cl_device_info,
                              param_value_size: libc::size_t,
                              param_value: *mut libc::c_void,
                              param_value_size_ret: *mut libc::size_t) -> cl_int;

    /* Context APIs */
    unsafe fn clCreateContext(properties: *cl_context_properties,
                              num_devices: cl_uint,
                              devices: *cl_device_id,
                              pfn_notify: extern unsafe fn (*libc::c_char, *libc::c_void, libc::size_t, *libc::c_void),
                              user_data: *libc::c_void,
                              errcode_ret: *cl_int) -> cl_context;
    unsafe fn clCreateContextFromType(properties: *cl_context_properties,
                                      device_type: cl_device_type,
                                      pfn_notify: extern unsafe fn (*libc::c_char, *libc::c_void, libc::size_t, *libc::c_void),
                                      user_data: *libc::c_void,
                                      errcode_ret: *cl_int) -> cl_context;
    unsafe fn clRetainContext(context: cl_context) -> cl_int;
    unsafe fn clReleaseContext(context: cl_context) -> cl_int;
    unsafe fn clGetContextInfo(context: cl_context,
                               param_name: cl_context_info,
                               param_value_size: libc::size_t,
                               param_value: *libc::c_void,
                               param_value_size_ret: *libc::size_t) -> cl_int;

    /* Command Queue APIs */
    unsafe fn clCreateCommandQueue(context: cl_context,
                                   device: cl_device_id,
                                   properties: cl_command_queue_properties,
                                   errcode_ret: *cl_int) -> cl_command_queue;
    unsafe fn clRetainCommandQueue(command_queue: cl_command_queue) -> cl_int;
    unsafe fn clReleaseCommandQueue(command_queue: cl_command_queue) -> cl_int;
    unsafe fn clGetCommandQueueInfo(command_queue: cl_command_queue,
                                    param_name: cl_command_queue_info,
                                    param_value_size: libc::size_t,
                                    param_value: *libc::c_void,
                                    param_value_size_ret: *libc::size_t) -> cl_int;
    unsafe fn clSetCommandQueueProperty(command_queue: cl_command_queue,
                                        properties: cl_command_queue_properties,
                                        enable: cl_bool,
                                        old_properties: *cl_command_queue_properties) -> cl_int; /* DEPRECATED: From 1.1 (Not thread safe) */

    /* Memory Object APIs */
    unsafe fn clCreateBuffer(context: cl_context,
                             flags: cl_mem_flags,
                             size: libc::size_t,
                             host_ptr: *libc::c_void,
                             errcode_ret: *cl_int) -> cl_mem;
    unsafe fn clCreateImage2D(context: cl_context,
                              flags: cl_mem_flags,
                              image_format: *cl_image_format,
                              image_width: libc::size_t,
                              image_height: libc::size_t,
                              image_row_pitch: libc::size_t,
                              host_ptr: *libc::c_void,
                              errcode_ret: *cl_int) -> cl_mem; /* DEPRECATED: From 1.2 */
    unsafe fn clCreateImage3D(context: cl_context,
                              flags: cl_mem_flags,
                              image_format: *cl_image_format,
                              image_width: libc::size_t,
                              image_height: libc::size_t,
                              image_depth: libc::size_t,
                              image_row_pitch: libc::size_t,
                              image_depth: libc::size_t,
                              image_row_pitch: libc::size_t,
                              image_slice_pitch: libc::size_t,
                              host_ptr: *libc::c_void,
                              errcode_ret: *cl_int) -> cl_mem; /* DEPRECATED: From 1.2 */
    unsafe fn clRetainMemObject(memobj: cl_mem) -> cl_int;
    unsafe fn clReleaseMemObject(memobj: cl_mem) -> cl_int;
    unsafe fn clGetSupportedImageFormats(context: cl_context,
                                         flags: cl_mem_flags,
                                         image_type: cl_mem_object_type,
                                         num_entries: cl_uint,
                                         image_formats: *cl_image_format,
                                         num_image_formats: *cl_uint) -> cl_int;
    unsafe fn clGetMemObjectInfo(memobj: cl_mem,
                                 param_name: cl_mem_info,
                                 param_value_size: libc::size_t,
                                 param_value: *libc::c_void,
                                 param_value_size_ret: *libc::size_t) -> cl_int;
    unsafe fn clGetImageInfo(image: cl_mem,
                             param_name: cl_image_info,
                             param_value_size: libc::size_t,
                             param_value: *libc::c_void,
                             param_value_size_ret: *libc::size_t) -> cl_int;

    /** Sampler APIs */
    unsafe fn clCreateSampler(context: cl_context,
                              normalize_coords: cl_bool,
                              addressing_mode: cl_addressing_mode,
                              filter_mode: cl_filter_mode,
                              errcode_ret: *cl_int) -> cl_sampler;
    unsafe fn clRetainSampler(sampler: cl_sampler) -> cl_int;
    unsafe fn clReleaseSampler(sampler: cl_sampler) ->cl_int;
    unsafe fn clGetSamplerInfo(sampler: cl_sampler,
                               param_name: cl_sampler_info,
                               param_value_size: libc::size_t,
                               param_value: *libc::c_void,
                               param_value_size_ret: *libc::size_t) -> cl_int;

    /* Program Object APIs */
    unsafe fn clCreateProgramWithSource(context: cl_context,
                                        count: cl_uint,
                                        strings: **libc::c_char,
                                        lengths: *libc::size_t,
                                        errcode_ret: *cl_int) -> cl_program;
    unsafe fn clCreateProgramWithBinary(context: cl_context,
                                        num_devices: cl_uint,
                                        device_list: *cl_device_id,
                                        lengths: *libc::size_t,
                                        binaries: **libc::c_uchar,
                                        binary_status: *cl_int,
                                        errcode_ret: *cl_int) -> cl_program;
    unsafe fn clRetainProgram(program: cl_program) -> cl_int;
    unsafe fn clReleaseProgram(program: cl_program) -> cl_int;
    unsafe fn clBuildProgram(program: cl_program,
                             num_devices: cl_uint,
                             device_list: *cl_device_id,
                             options: *libc::c_char,
                             pfn_notify: extern unsafe fn (cl_program, *libc::c_void),
                             user_data: *libc::c_void) -> cl_int;
    unsafe fn clUnloadCompiler() -> cl_int; /* DEPRECATED: From 1.2 */
    unsafe fn clGetProgramInfo(program: cl_program,
                               param_name: cl_program_info,
                               param_value_size: libc::size_t,
                               param_value: *libc::c_void,
                               param_value_size_ret: *libc::size_t) -> cl_int;
    unsafe fn clGetProgramBuildInfo(program: cl_program,
                                    device: cl_device_id,
                                    param_name: cl_program_info,
                                    param_value_size: libc::size_t,
                                    param_value: *libc::c_void,
                                    param_value_size_ret: *libc::size_t) -> cl_int;

    /* Kernel Object APIs */
    unsafe fn clCreateKernel(program: cl_program,
                             kernel_name: *libc::c_char,
                             errcode_ret: *cl_int) -> cl_kernel;
    unsafe fn clCreateKernelsInProgram(program: cl_program,
                                       num_kernels: cl_uint,
                                       kernels: *cl_kernel,
                                       num_kernels_ret: *cl_uint) -> cl_int;
    unsafe fn clRetainKernel(kernel: cl_kernel) -> cl_int;
    unsafe fn clReleaseKernel(kernel: cl_kernel) -> cl_int;
    unsafe fn clSetKernelArg(kernel: cl_kernel,
                             arg_index: cl_uint,
                             arg_size: libc::size_t,
                             arg_value: *libc::c_void) -> cl_int;
    unsafe fn clGetKernelInfo(kernel: cl_kernel,
                              param_name: cl_kernel_info,
                              param_value_size: libc::size_t,
                              param_value: *libc::c_void,
                              param_value_size_ret: *libc::size_t) -> cl_int;
    unsafe fn clGetKernelWorkGroupInfo(kernel: cl_kernel,
                                       device: cl_device_id,
                                       param_name: cl_kernel_work_group_info,
                                       param_value_size: libc::size_t,
                                       param_value: *libc::c_void,
                                       param_value_size_ret: *libc::size_t) -> cl_int;

    /* Event Object APIs */
    unsafe fn clWaitForEvents(num_events: cl_uint,
                              event_list: *cl_event) -> cl_int;
    unsafe fn clGetEventInfo(event: cl_event,
                             param_name: cl_event_info,
                             param_value_size: libc::size_t,
                             param_value: *libc::c_void,
                             param_value_size_ret: *libc::size_t) -> cl_int;
    unsafe fn clRetainEvent(event: cl_event) -> cl_int;
    unsafe fn clReleaseEvent(event: cl_event) -> cl_int;

    /* Profiling APIs */
    unsafe fn clGetEventProfilingInfo(event: cl_event,
                                      param_name: cl_profiling_info,
                                      param_value_size: libc::size_t,
                                      param_value: *libc::c_void,
                                      param_value_size_ret: *libc::size_t) -> cl_int;

    /* Flush and Finish APIs */
    unsafe fn clFlush(command_queue: cl_command_queue) -> cl_int;
    unsafe fn clFinish(command_queue: cl_command_queue) -> cl_int;

    /* Enqueued Commands APIs */
    unsafe fn clEnqueueReadBuffer(command_queue: cl_command_queue,
                                  buffer: cl_mem,
                                  blocking_read: cl_bool,
                                  offset: libc::size_t,
                                  cb: libc::size_t,
                                  ptr: *libc::c_void,
                                  num_events_in_wait_list: cl_uint,
                                  event_wait_list: *cl_event,
                                  event: *cl_event) -> cl_int;
    unsafe fn clEnqueueWriteBuffer(command_queue: cl_command_queue,
                                   buffer: cl_mem,
                                   blocking_write: cl_bool,
                                   offset: libc::size_t,
                                   cb: libc::size_t,
                                   ptr: *libc::c_void,
                                   num_events_in_wait_list: cl_uint,
                                   event_wait_list: *cl_event,
                                   event: *cl_event) -> cl_int;
    unsafe fn clEnqueueCopyBuffer(command_queue: cl_command_queue,
                                  src_buffer: cl_mem,
                                  dst_buffer: cl_mem,
                                  src_offset: libc::size_t,
                                  dst_offset: libc::size_t,
                                  cb: libc::size_t,
                                  num_events_in_wait_list: cl_uint,
                                  event_wait_list: *cl_event,
                                  event: *cl_event) -> cl_int;
    unsafe fn clEnqueueReadImage(command_queue: cl_command_queue,
                                 image: cl_mem,
                                 blocking_read: cl_bool,
                                 origin: *libc::size_t,
                                 region: *libc::size_t,
                                 row_pitch: libc::size_t,
                                 slice_pitch: libc::size_t,
                                 ptr: *libc::c_void,
                                 num_events_in_wait_list: cl_uint,
                                 event_wait_list: *cl_event,
                                 event: *cl_event) -> cl_int;
    unsafe fn clEnqueueWriteImage(command_queue: cl_command_queue,
                                  image: cl_mem,
                                  blocking_write: cl_bool,
                                  origin: *libc::size_t,
                                  region: *libc::size_t,
                                  input_row_pitch: libc::size_t,
                                  input_slice_pitch: libc::size_t,
                                  ptr: *libc::c_void,
                                  num_events_in_wait_list: cl_uint,
                                  event_wait_list: *cl_event,
                                  event: *cl_event) -> cl_int;
    unsafe fn clEnqueueCopyImage(command_queue: cl_command_queue,
                                 src_image: cl_mem,
                                 dst_image: cl_mem,
                                 src_origin: *libc::size_t,
                                 dst_origin: *libc::size_t,
                                 region: *libc::size_t,
                                 num_events_in_wait_list: cl_uint,
                                 event_wait_list: *cl_event,
                                 event: *cl_event) -> cl_int;
    unsafe fn clEnqueueCopyImageToBuffer(command_queue: cl_command_queue,
                                         src_image: cl_mem,
                                         dst_buffer: cl_mem,
                                         src_origin: *libc::size_t,
                                         region: *libc::size_t,
                                         dst_offset: libc::size_t,
                                         num_events_in_wait_list: cl_uint,
                                         event_wait_list: *cl_event,
                                         event: *cl_event) -> cl_int;
    unsafe fn clEnqueueCopyBufferToImage(command_queue: cl_command_queue,
                                         src_buffer: cl_mem,
                                         dst_image: cl_mem,
                                         src_offset: libc::size_t,
                                         dst_origin: *libc::size_t,
                                         region: *libc::size_t,
                                         num_events_in_wait_list: cl_uint,
                                         event_wait_list: *cl_event,
                                         event: *cl_event) -> cl_int;
    unsafe fn clEnqueueMapBuffer(command_queue: cl_command_queue,
                                 buffer: cl_mem,
                                 blocking_map: cl_bool,
                                 map_flags: cl_map_flags,
                                 offset: libc::size_t,
                                 cb: libc::size_t,
                                 num_events_in_wait_list: cl_uint,
                                 event_wait_list: *cl_event,
                                 event: *cl_event,
                                 errorcode_ret: *cl_int);
    unsafe fn clEnqueueMapImage(command_queue: cl_command_queue,
                                image: cl_mem,
                                blocking_map: cl_bool,
                                map_flags: cl_map_flags,
                                origin: *libc::size_t,
                                region: *libc::size_t,
                                image_row_pitch: libc::size_t,
                                image_slice_pitch: libc::size_t,
                                num_events_in_wait_list: cl_uint,
                                event_wait_list: *cl_event,
                                event: *cl_event,
                                errorcode_ret: *cl_int);
    unsafe fn clEnqueueUnmapMemObject(command_queue: cl_command_queue,
                                      memobj: cl_mem,
                                      mapped_ptr: *libc::c_void,
                                      num_events_in_wait_list: cl_uint,
                                      event_wait_list: *cl_event,
                                      event: *cl_event) -> cl_int;
    unsafe fn clEnqueueNDRangeKernel(command_queue: cl_command_queue,
                                     kernel: cl_kernel,
                                     work_dim: cl_uint,
                                     global_work_offset: *libc::size_t,
                                     global_work_size: *libc::size_t,
                                     local_work_size: *libc::size_t,
                                     num_events_in_wait_list: cl_uint,
                                     event_wait_list: *cl_event,
                                     event: *cl_event) -> cl_int;
    unsafe fn clEnqueueTask(command_queue: cl_command_queue,
                            kernel: cl_kernel,
                            num_events_in_wait_list: cl_uint,
                            event_wait_list: *cl_event,
                            event: *cl_event) -> cl_int;
    unsafe fn clEnqueueNativeKernel(command_queue: cl_command_queue,
                                    user_func: extern unsafe fn (*libc::c_void),
                                    args: *libc::c_void,
                                    cb_args: libc::size_t,
                                    num_mem_objects: cl_uint,
                                    mem_list: *cl_mem,
                                    args_mem_loc: **libc::c_void,
                                    num_events_in_wait_list: cl_uint,
                                    event_wait_list: *cl_event,
                                    event: *cl_event) -> cl_int;
    unsafe fn clEnqueueMarker(command_queue: cl_command_queue,
                              event: *cl_event) -> cl_int; /* DEPRECATED: From 1.2 */
    unsafe fn clEnqueueWaitForEvents(command_queue: cl_command_queue,
                                     num_events: cl_uint,
                                     event_list: *cl_event) -> cl_int; /* DEPRECATED: From 1.2 */
    unsafe fn clEnqueueBarrier(command_queue: cl_command_queue) -> cl_int; /* DEPRECATED: From 1.2 */
                                
    /* Extension function access
     *
     * Returns the extension function address for the given function name,
     * or NULL if a valid function can not be found. The client must
     * check to make sure the address is not NULL, before using or
     * or calling the returned function address.
     */
    unsafe fn clGetExtensionFunctionAddress(func_name: *libc::c_char); /* DEPRECATED: From 1.2 */
}

/*
 * OpenCL 1.1 cl_platform.h
 */

/* TODO: More constant macros */

/*
 * OpenCL 1.1 cl.h
 */

pub struct cl_buffer_region {
    origin: libc::size_t,
    size: libc::size_t
}

/* Error Codes */
cl_error!(CL_MISALIGNED_SUB_BUFFER_OFFSET = -13)
cl_error!(CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST = -14)

cl_error!(CL_INVALID_PROPERTY = -64)

/* OpenCL Version */
cl_constant!(cl_int, CL_VERSION_1_1 = 1)

/* cl_device_info */
cl_constant!(cl_device_info, CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF = 0x1034)
cl_constant!(cl_device_info, CL_DEVICE_HOST_UNIFIED_MEMORY = 0x1035)
cl_constant!(cl_device_info, CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR = 0x1036)
cl_constant!(cl_device_info, CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT = 0x1037)
cl_constant!(cl_device_info, CL_DEVICE_NATIVE_VECTOR_WIDTH_INT = 0x1038)
cl_constant!(cl_device_info, CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG = 0x1039)
cl_constant!(cl_device_info, CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT = 0x103A)
cl_constant!(cl_device_info, CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE = 0x103B)
cl_constant!(cl_device_info, CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF = 0x103C)
cl_constant!(cl_device_info, CL_DEVICE_OPENCL_C_VERSION = 0x103D)

/* cl_device_fp_config - bitfield */
cl_constant!(cl_device_fp_config, CL_FP_SOFT_FLOAT = (1 << 6))

/* cl_context_info  */
cl_constant!(cl_context_info, CL_CONTEXT_NUM_DEVICES = 0x1083)

/* cl_channel_order */
cl_constant!(cl_channel_order, CL_Rx = 0x10BA)
cl_constant!(cl_channel_order, CL_RGx = 0x10BB)
cl_constant!(cl_channel_order, CL_RGBx = 0x10BC)

/* cl_mem_info */
cl_constant!(cl_int, CL_MEM_ASSOCIATED_MEMOBJECT = 0x1107)
cl_constant!(cl_int, CL_MEM_OFFSET = 0x1108)

/* cl_addressing_mode */
cl_constant!(cl_addressing_mode, CL_ADDRESS_MIRRORED_REPEAT = 0x1134)

/* cl_kernel_work_group_info */
cl_constant!(cl_kernel_work_group_info, CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE = 0x11B3)
cl_constant!(cl_kernel_work_group_info, CL_KERNEL_PRIVATE_MEM_SIZE = 0x11B4)

/* cl_event_info  */
cl_constant!(cl_event_info, CL_EVENT_CONTEXT = 0x11D4)

/* cl_command_type */
cl_constant!(cl_command_type, CL_COMMAND_READ_BUFFER_RECT = 0x1201)
cl_constant!(cl_command_type, CL_COMMAND_WRITE_BUFFER_RECT = 0x1202)
cl_constant!(cl_command_type, CL_COMMAND_COPY_BUFFER_RECT = 0x1203)
cl_constant!(cl_command_type, CL_COMMAND_USER = 0x1204)

/* cl_buffer_create_type  */
cl_constant!(cl_buffer_create_type, CL_BUFFER_CREATE_TYPE_REGION = 0x1220)

#[nolink]
pub extern {
    /* Memory Object APIs */
    unsafe fn clCreateSubBuffer(buffer: cl_mem,
                               flags: cl_mem_flags,
                               buffer_create_type: cl_buffer_create_type,
                               buffer_create_info: *libc::c_void,
                               errcode_ret: *cl_int) -> cl_mem;
    unsafe fn clSetMemObjectDestructorCallback(memobj: cl_mem,
                                               pfn_notify: extern unsafe fn (cl_mem, *libc::c_void),
                                               user_data: *libc::c_void) -> cl_int;

    /* Event Object APIs  */
    unsafe fn clCreateUserEvent(context: cl_context,
                                errcode_ret: *cl_int) -> cl_event;
    unsafe fn clSetUserEventStatus(event: cl_event,
                                   execution_status: cl_int) -> cl_int;
    unsafe fn clSetEventCallback(event: cl_event,
                                 command_exec_callback_type: cl_int,
                                 pfn_notify: extern unsafe fn (cl_event, cl_int, *libc::c_void),
                                 user_data: *libc::c_void) -> cl_int;

    /* Enqueued Commands APIs */
    unsafe fn clEnqueueReadBufferRect(command_queue: cl_command_queue,
                                      buffer: cl_mem,
                                      blocking_read: cl_bool,
                                      buffer_origin: *libc::size_t,
                                      host_origin: *libc::size_t,
                                      region: *libc::size_t,
                                      buffer_row_pitch: libc::size_t,
                                      buffer_slice_pitch: libc::size_t,
                                      host_row_pitch: libc::size_t,
                                      host_slice_pitch: libc::size_t,
                                      ptr: *libc::c_void,
                                      num_events_in_wait_list: cl_uint,
                                      event_wait_list: *cl_event,
                                      event: *cl_event) -> cl_int;
    unsafe fn clEnqueueWriteBufferRect(command_queue: cl_command_queue,
                                       blocking_write: cl_bool,
                                       buffer_origin: *libc::size_t,
                                       host_origin: *libc::size_t,
                                       region: *libc::size_t,
                                       buffer_row_pitch: libc::size_t,
                                       buffer_slice_pitch: libc::size_t,
                                       host_row_pitch: libc::size_t,
                                       host_slice_pitch: libc::size_t,
                                       ptr: *libc::c_void,
                                       num_events_in_wait_list: cl_uint,
                                       event_wait_list: *cl_event,
                                       event: *cl_event) -> cl_int;
    unsafe fn clEnqueueCopyBufferRect(command_queue: cl_command_queue,
                                      src_buffer: cl_mem,
                                      dst_buffer: cl_mem,
                                      src_origin: *libc::size_t,
                                      dst_origin: *libc::size_t,
                                      region: *libc::size_t,
                                      src_row_pitch: libc::size_t,
                                      src_slice_pitch: libc::size_t,
                                      dst_row_pitch: libc::size_t,
                                      dst_slice_pitch: libc::size_t,
                                      num_events_in_wait_list: cl_uint,
                                      event_wait_list: *cl_event,
                                      event: *cl_event) -> cl_int;
}

/*
 * OpenCL 1.2 platform.h
 */

/*
 * OpenCL 1.2 cl.h
 */

cl_type!(cl_device_partition_property, libc::intptr_t)
cl_type!(cl_device_affinity_domain, cl_bitfield)

cl_type!(cl_mem_migration_flags, cl_bitfield)
cl_type!(cl_program_binary_type, cl_uint)

cl_type!(cl_kernel_arg_info, cl_uint)
cl_type!(cl_kernel_arg_address_qualifier, cl_uint)
cl_type!(cl_kernel_arg_access_qualifier, cl_uint)
cl_type!(cl_kernel_arg_type_qualifier, cl_bitfield)

pub struct cl_image_desc {
    image_type: cl_mem_object_type,
    image_width: libc::size_t,
    image_height: libc::size_t,
    image_depth: libc::size_t,
    image_array_size: libc::size_t,
    image_row_pitch: libc::size_t,
    image_slice_pitch: libc::size_t,
    num_mip_levels: cl_uint,
    num_samples: cl_uint,
    buffer: cl_mem
}

/* Error Codes */
cl_error!(CL_COMPILE_PROGRAM_FAILURE = -15)
cl_error!(CL_LINKER_NOT_AVAILABLE = -16)
cl_error!(CL_LINK_PROGRAM_FAILURE = -17)
cl_error!(CL_DEVICE_PARTITION_FAILED = -18)
cl_error!(CL_KERNEL_ARG_INFO_NOT_AVAILABLE = -19)

cl_error!(CL_INVALID_IMAGE_DESCRIPTOR = -65)
cl_error!(CL_INVALID_COMPILER_OPTIONS = -66)
cl_error!(CL_INVALID_LINKER_OPTIONS = -67)
cl_error!(CL_INVALID_DEVICE_PARTITION_COUNT = -68)

/* OpenCL Version */
cl_constant!(cl_int, CL_VERSION_1_2 = 1)

/* cl_bool */
cl_constant!(cl_bool, CL_BLOCKING = CL_TRUE)
cl_constant!(cl_bool, CL_NON_BLOCKING = CL_FALSE)

/* cl_device_type - bitfield */
cl_constant!(cl_bitfield, CL_DEVICE_TYPE_CUSTOM = (1 << 4))

/* cl_device_info */
cl_constant!(cl_device_info, CL_DEVICE_DOUBLE_FP_CONFIG = 0x1032)
cl_constant!(cl_device_info, CL_DEVICE_LINKER_AVAILABLE = 0x103E)
cl_constant!(cl_device_info, CL_DEVICE_BUILT_IN_KERNELS = 0x103F)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE_MAX_BUFFER_SIZE = 0x1040)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE_MAX_ARRAY_SIZE = 0x1041)
cl_constant!(cl_device_info, CL_DEVICE_PARENT_DEVICE = 0x1042)
cl_constant!(cl_device_info, CL_DEVICE_PARTITION_MAX_SUB_DEVICES = 0x1043)
cl_constant!(cl_device_info, CL_DEVICE_PARTITION_PROPERTIES = 0x1044)
cl_constant!(cl_device_info, CL_DEVICE_PARTITION_AFFINITY_DOMAIN = 0x1045)
cl_constant!(cl_device_info, CL_DEVICE_PARTITION_TYPE = 0x1046)
cl_constant!(cl_device_info, CL_DEVICE_REFERENCE_COUNT = 0x1047)
cl_constant!(cl_device_info, CL_DEVICE_PREFERRED_INTEROP_USER_SYNC = 0x1048)
cl_constant!(cl_device_info, CL_DEVICE_PRINTF_BUFFER_SIZE = 0x1049)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE_PITCH_ALIGNMENT = 0x104A)
cl_constant!(cl_device_info, CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT = 0x104B)

/* cl_device_fp_config - bitfield */
cl_constant!(cl_device_fp_config, CL_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT = (1 << 7))

/* cl_context_properties */
cl_constant!(cl_context_properties, CL_CONTEXT_INTEROP_USER_SYNC = 0x1085)

/* cl_device_partition_property */
cl_constant!(cl_device_partition_property, CL_DEVICE_PARTITION_EQUALLY = 0x1086)
cl_constant!(cl_device_partition_property, CL_DEVICE_PARTITION_BY_COUNTS = 0x1087)
cl_constant!(cl_device_partition_property, CL_DEVICE_PARTITION_BY_COUNTS_LIST_END = 0x0)
cl_constant!(cl_device_partition_property, CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN = 0x1088)

/* cl_device_affinity_domain */
cl_constant!(cl_device_affinity_domain, CL_DEVICE_AFFINITY_DOMAIN_NUMA = (1 << 0))
cl_constant!(cl_device_affinity_domain, CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE = (1 << 1))
cl_constant!(cl_device_affinity_domain, CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE = (1 << 2))
cl_constant!(cl_device_affinity_domain, CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE = (1 << 3))
cl_constant!(cl_device_affinity_domain, CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE = (1 << 4))
cl_constant!(cl_device_affinity_domain, CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE = (1 << 5))

/* cl_mem_flags - bitfield */
cl_constant!(cl_mem_flags, CL_MEM_HOST_WRITE_ONLY = (1 << 7))
cl_constant!(cl_mem_flags, CL_MEM_HOST_READ_ONLY = (1 << 8))
cl_constant!(cl_mem_flags, CL_MEM_HOST_NO_ACCESS = (1 << 9))

/* cl_mem_migration_flags - bitfield */
cl_constant!(cl_mem_migration_flags, CL_MIGRATE_MEM_OBJECT_HOST = (1 << 0))
cl_constant!(cl_mem_migration_flags, CL_MIGRATE_MEM_OBJECT_CONTENT_UNDEFINED = (1 << 1))

/* cl_channel_order */
cl_constant!(cl_channel_order, CL_DEPTH = 0x10BD)
cl_constant!(cl_channel_order, CL_DEPTH_STENCIL = 0x10BE)

/* cl_channel_type */
cl_constant!(cl_channel_type, CL_UNORM_INT24 = 0x10DF)

/* cl_mem_object_type */
cl_constant!(cl_mem_object_type, CL_MEM_OBJECT_IMAGE2D_ARRAY = 0x10F3)
cl_constant!(cl_mem_object_type, CL_MEM_OBJECT_IMAGE1D = 0x10F4)
cl_constant!(cl_mem_object_type, CL_MEM_OBJECT_IMAGE1D_ARRAY = 0x10F5)
cl_constant!(cl_mem_object_type, CL_MEM_OBJECT_IMAGE1D_BUFFER = 0x10F6)

/* cl_image_info */
cl_constant!(cl_image_info, CL_IMAGE_ARRAY_SIZE = 0x1117)
cl_constant!(cl_image_info, CL_IMAGE_BUFFER = 0x1118)
cl_constant!(cl_image_info, CL_IMAGE_NUM_MIP_LEVELS = 0x1119)
cl_constant!(cl_image_info, CL_IMAGE_NUM_SAMPLES = 0x111A)

/* cl_map_flags - bitfield */
cl_constant!(cl_map_flags, CL_MAP_WRITE_INVALIDATE_REGION = (1 << 2))

/* cl_program_info */
cl_constant!(cl_program_info, CL_PROGRAM_NUM_KERNELS = 0x1167)
cl_constant!(cl_program_info, CL_PROGRAM_KERNEL_NAMES = 0x1168)

/* cl_program_build_info */
cl_constant!(cl_program_build_info, CL_PROGRAM_BINARY_TYPE = 0x1184)

/* cl_program_binary_type */
cl_constant!(cl_program_binary_type, CL_PROGRAM_BINARY_TYPE_NONE = 0x0)
cl_constant!(cl_program_binary_type, CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT = 0x1)
cl_constant!(cl_program_binary_type, CL_PROGRAM_BINARY_TYPE_LIBRARY = 0x2)
cl_constant!(cl_program_binary_type, CL_PROGRAM_BINARY_TYPE_EXECUTABLE = 0x4)

/* cl_kernel_info */
cl_constant!(cl_kernel_info, CL_KERNEL_ATTRIBUTES = 0x1195)

/* cl_kernel_arg_info */
cl_constant!(cl_kernel_arg_info, CL_KERNEL_ARG_ADDRESS_QUALIFIER = 0x1196)
cl_constant!(cl_kernel_arg_info, CL_KERNEL_ARG_ACCESS_QUALIFIER = 0x1197)
cl_constant!(cl_kernel_arg_info, CL_KERNEL_ARG_TYPE_NAME = 0x1198)
cl_constant!(cl_kernel_arg_info, CL_KERNEL_ARG_TYPE_QUALIFIER = 0x1199)
cl_constant!(cl_kernel_arg_info, CL_KERNEL_ARG_NAME = 0x119A)
    
/* cl_kernel_arg_address_qualifier */
cl_constant!(cl_kernel_arg_address_qualifier, CL_KERNEL_ARG_ADDRESS_GLOBAL = 0x119B)
cl_constant!(cl_kernel_arg_address_qualifier, CL_KERNEL_ARG_ADDRESS_LOCAL = 0x119C)
cl_constant!(cl_kernel_arg_address_qualifier, CL_KERNEL_ARG_ADDRESS_CONSTANT = 0x119D)
cl_constant!(cl_kernel_arg_address_qualifier, CL_KERNEL_ARG_ADDRESS_PRIVATE = 0x119E)

/* cl_kernel_arg_access_qualifier */
cl_constant!(cl_kernel_arg_access_qualifier, CL_KERNEL_ARG_ACCESS_READ_ONLY = 0x11A0)
cl_constant!(cl_kernel_arg_access_qualifier, CL_KERNEL_ARG_ACCESS_WRITE_ONLY = 0x11A1)
cl_constant!(cl_kernel_arg_access_qualifier, CL_KERNEL_ARG_ACCESS_READ_WRITE = 0x11A2)
cl_constant!(cl_kernel_arg_access_qualifier, CL_KERNEL_ARG_ACCESS_NONE = 0x11A3)

/* cl_kernel_arg_type_qualifier */
cl_constant!(cl_kernel_arg_type_qualifier, CL_KERNEL_ARG_TYPE_NONE = 0)
cl_constant!(cl_kernel_arg_type_qualifier, CL_KERNEL_ARG_TYPE_CONST = (1 << 0))
cl_constant!(cl_kernel_arg_type_qualifier, CL_KERNEL_ARG_TYPE_RESTRICT = (1 << 1))
cl_constant!(cl_kernel_arg_type_qualifier, CL_KERNEL_ARG_TYPE_VOLATILE = (1 << 2))

/* cl_kernel_work_group_info */
cl_constant!(cl_kernel_work_group_info, CL_KERNEL_GLOBAL_WORK_SIZE = 0x11B5)

/* cl_command_type */
cl_constant!(cl_command_type, CL_COMMAND_BARRIER = 0x1205)
cl_constant!(cl_command_type, CL_COMMAND_MIGRATE_MEM_OBJECTS = 0x1206)
cl_constant!(cl_command_type, CL_COMMAND_FILL_BUFFER = 0x1207)
cl_constant!(cl_command_type, CL_COMMAND_FILL_IMAGE = 0x1208)

#[nolink]
pub extern {
    /* Device APIs */
    unsafe fn clCreateSubDevices(in_device: cl_device_id,
                                 properties: *cl_device_partition_property,
                                 num_devices: cl_uint,
                                 out_devices: *cl_device_id,
                                 num_devices_ret: *cl_uint) -> cl_int;
    unsafe fn clRetainDevice(device: cl_device_id) -> cl_int;
    unsafe fn clReleaseDevice(device: cl_device_id) -> cl_int;

    /* Memory Object APIs */
    unsafe fn clCreateImage(context: cl_context,
                            flags: cl_mem_flags,
                            image_format: *cl_image_format,
                            image_desc: *cl_image_desc,
                            host_ptr: *libc::c_void,
                            errcode_ret: *cl_int) -> cl_mem;

    /* Program Object APIs */
    unsafe fn clCreateProgramWithBuiltInKernels(context: cl_context,
                                                num_devices: cl_uint,
                                                device_list: *cl_device_id,
                                                kernel_names: *libc::c_char,
                                                errcode_ret: *cl_int) -> cl_program;
    unsafe fn clCompileProgram(program: cl_program,
                               num_devices: cl_uint,
                               device_list: *cl_device_id,
                               options: *libc::c_char,
                               num_input_headers: cl_uint,
                               input_headers: *cl_program,
                               header_include_names: **libc::c_char,
                               pfn_notify: extern fn(program: cl_program, user_data: *libc::c_void),
                               user_data: *libc::c_void) -> cl_int;
    unsafe fn clLinkProgram(context: cl_context,
                            num_devices: cl_uint,
                            device_list: *cl_device_id,
                            options: *libc::c_char,
                            num_input_programs: cl_uint,
                            input_programs: *cl_program,
                            pfn_notify: extern fn(program: cl_program, user_data: *libc::c_void),
                            errcode_ret: *cl_int) -> cl_program;
    unsafe fn clUnloadPlatformCompiler(platform: cl_platform_id) -> cl_int;
    unsafe fn clGetKernelArgInfo(kernel: cl_kernel,
                                 arg_index: cl_uint,
                                 param_name: cl_kernel_arg_info,
                                 param_value_size: libc::size_t,
                                 param_value: *libc::c_void,
                                 param_value_size_ret: *libc::size_t) -> cl_int;

    /* Enqueued Commands APIs */
    unsafe fn clEnqueueFillBuffer(command_queue: cl_command_queue,
                                  buffer: cl_mem,
                                  pattern: *libc::c_void,
                                  pattern_size: libc::size_t,
                                  offset: libc::size_t,
                                  size: libc::size_t,
                                  num_events_in_wait_list: cl_uint,
                                  event_wait_list: *cl_event,
                                  event: *cl_event) -> cl_int;
    unsafe fn clEnqueueFillImage(command_queue: cl_command_queue,
                                 image: cl_mem,
                                 fill_color: *libc::c_void,
                                 origin: *libc::size_t,
                                 region: *libc::size_t,
                                 size: libc::size_t,
                                 num_events_in_wait_list: cl_uint,
                                 event_wait_list: *cl_event,
                                 event: *cl_event) -> cl_int;
    unsafe fn clEnqueueMigrateMemObjects(command_queue: cl_command_queue,
                                         num_mem_objects: cl_uint,
                                         mem_objects: *cl_mem,
                                         flags: cl_mem_migration_flags,
                                         num_events_in_wait_list: cl_uint,
                                         event_wait_list: *cl_event,
                                         event: *cl_event) -> cl_int;
    unsafe fn clEnqueueMarkerWithWaitList(command_queue: cl_command_queue,
                                          num_events_in_wait_list: cl_uint,
                                          event_wait_list: *cl_event,
                                          event: *cl_event) -> cl_int;
    unsafe fn clEnqueueBarrierWithWaitList(command_queue: cl_command_queue,
                                           num_events_in_wait_list: cl_uint,
                                           event_wait_list: *cl_event,
                                           event: *cl_event) -> cl_int;

    /* Extension function access
     *
     * Returns the extension function address for the given function name,
     * or NULL if a valid function can not be found.  The client must
     * check to make sure the address is not NULL, before using or 
     * calling the returned function address.
     */
    unsafe fn clGetExtensionFunctionAddressForPlatform(platform: cl_platform_id,
                                                       func_name: *libc::c_char) -> *libc::c_void;
}

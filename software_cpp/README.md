## Rectify issue with FreeRTOS-Kernel CMake and FreeRTOSConfig

```cmake
set(FREERTOS_CONFIG_FILE_DIRECTORY "${CMAKE_SOURCE_DIR}/lib" CACHE STRING "")

set(FREERTOS_PORT "GCC_ARM_CM0" CACHE STRING "")
```
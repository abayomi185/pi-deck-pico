cmake_minimum_required(VERSION 3.17)

include ($ENV{PICO_SDK_PATH}/pico_sdk_init.cmake)

# Project name and supported file extensions
project(PiDeckPico C CXX ASM)
# C and C++ version standards
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# set(PICO_CXX_ENABLE_EXCEPTIONS 1)

pico_sdk_init()


file (GLOB_RECURSE Addon_CPP_SOURCES CONFIGURE_DEPENDS "include/*.cpp")
file (GLOB_RECURSE Addon_HPP_HEADERS CONFIGURE_DEPENDS "include/*.hpp")

set (Addon_INCLUDE_DIRS "")
foreach (_headerFile ${Addon_HPP_HEADERS})
    get_filename_component(_dir ${_headerFile} PATH)
    list (APPEND Addon_INCLUDE_DIRS ${_dir})
endforeach()
list (REMOVE_DUPLICATES Addon_INCLUDE_DIRS)


add_executable(${PROJECT_NAME} ${PROJECT_SOURCE_DIR}/src/main.cpp ${Addon_CPP_SOURCES})
# add_executable(${PROJECT_NAME} ${PROJECT_SOURCE_DIR}/src/main.cpp)

add_subdirectory(lib/FreeRTOS-Kernel)
# add_subdirectory(lib/pico-ssd1306)



# For implementations
target_sources(${PROJECT_NAME}
    PUBLIC
        ${CMAKE_CURRENT_LIST_DIR}/lib/RPi-Pico-SSD1306-library/SSD1306.cpp
        ${CMAKE_CURRENT_LIST_DIR}/lib/RPi-Pico-SSD1306-library/GFX.cpp
)

# For header files
target_include_directories(${PROJECT_NAME}
    PUBLIC
    ${PROJECT_SOURCE_DIR}/lib/RPi-Pico-SSD1306-library
    ${PROJECT_SOURCE_DIR}/lib/FreeRTOS-Kernel/include
    ${PROJECT_SOURCE_DIR}/lib/FreeRTOS-Kernel/portable/GCC/ARM_CM0
    ${Addon_INCLUDE_DIRS}
)

include_directories(
    ${PROJECT_SOURCE_DIR}/include
    # ${PROJECT_SOURCE_DIR}/include/Display
    ${PROJECT_SOURCE_DIR}/include/assets
    ${PROJECT_SOURCE_DIR}/src
    ${PROJECT_SOURCE_DIR}/lib
)

# Link with the pico stdlib
target_link_libraries(${PROJECT_NAME} pico_stdlib hardware_adc hardware_i2c freertos_kernel)
# target_link_libraries(${PROJECT_NAME} pico_stdlib hardware_adc pico_ssd1306 hardware_i2c freertos_kernel)

# Create map/bin/hex/uf2 files
pico_add_extra_outputs(${PROJECT_NAME})

# Enable USB serial
pico_enable_stdio_usb(${PROJECT_NAME} 1)
pico_enable_stdio_uart(${PROJECT_NAME} 0)
//
// From https://github.com/Klrohias/unimage-native/blob/main/src/CBinding.h 
//

#include <stdint.h>

void* unimage_processor_create();
void unimage_processor_free(void* handle);

void unimage_processor_load_raw(void* handle, uint8_t* data, int32_t width, int32_t height, uint8_t format);
uint8_t unimage_processor_load(void* handle, uint8_t* data, uint32_t length);
int32_t unimage_processor_get_width(void* handle);
int32_t unimage_processor_get_height(void* handle);
uint8_t unimage_processor_get_format(void* handle);
uint8_t unimage_processor_copy_to_memory(void* handle, void* buffer);
uint8_t unimage_processor_resize(void* handle, int32_t width, int32_t height);
const char* unimage_processor_get_error_message(void* handle);
uint8_t* unimage_processor_get_buffer(void* handle);
uint8_t unimage_processor_copy_from(void* handle, void* unimage);
uint8_t unimage_processor_clip(void* handle, int32_t x, int32_t y, int32_t width, int32_t height);
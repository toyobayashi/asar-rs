#ifndef ASAR_RS_H_
#define ASAR_RS_H_

#include <stddef.h>

#ifdef _MSC_VER
  #if _MSC_VER < 1910  // MSVC 2017-
    #error MSVC 2017 or later is required.
  #endif
#endif

#ifdef __cplusplus
  #ifndef EXTERN_C
    #define EXTERN_C extern "C"
  #endif
  #ifndef EXTERN_C_START
    #define EXTERN_C_START extern "C" {
  #endif
  #ifndef EXTERN_C_END
    #define EXTERN_C_END }
  #endif
#else
  #ifndef EXTERN_C
    #define EXTERN_C
  #endif
  #ifndef EXTERN_C_START
    #define EXTERN_C_START
  #endif
  #ifndef EXTERN_C_END
    #define EXTERN_C_END
  #endif
#endif

#if defined(WIN32) || defined(_WIN32) || defined(__CYGWIN__) || defined(__MINGW__)  // NOLINT
  #ifdef ASAR_BUILD_DLL
    #ifdef __GNUC__
      #define _ASAR_EXPORT __attribute__((dllexport))
    #else
      #define _ASAR_EXPORT __declspec(dllexport)
    #endif
  #else
    #ifdef ASAR_USE_DLL
      #ifdef __GNUC__
        #define _ASAR_EXPORT __attribute__((dllimport))
      #else
        #define _ASAR_EXPORT __declspec(dllimport)
      #endif
    #else
      #define _ASAR_EXPORT
    #endif
  #endif
  #define _ASAR_LOCAL
#else
  #if __GNUC__ >= 4 && defined(ASAR_BUILD_DLL)
    #define _ASAR_EXPORT __attribute__((visibility("default")))
    #define _ASAR_LOCAL  __attribute__((visibility("hidden")))
  #else
    #define _ASAR_EXPORT
    #define _ASAR_LOCAL
  #endif
#endif

#ifndef ASAR_CALL
  #if defined(WIN32) || defined(_WIN32)
  #define ASAR_CALL /* __stdcall */
  #else
  #define ASAR_CALL /* __cdecl */
  #endif
#endif

#if defined(_MSC_VER)
  #define ASAR_API(ret_type) EXTERN_C _ASAR_EXPORT ret_type ASAR_CALL
#elif defined(__EMSCRIPTEN__)
  #define ASAR_API(ret_type) \
    EXTERN_C _ASAR_EXPORT ASAR_CALL ret_type __attribute__((used))
#else
  #define ASAR_API(ret_type) EXTERN_C _ASAR_EXPORT ASAR_CALL ret_type
#endif

typedef enum asar_status {
  success,
  invalid_arg,
  invalid_header_size,
  invalid_header,
  expect_file_node,
  expect_dir_node,
  file_too_large,
  unknown_offset,
  no_such_entry,
  relative_path,
  bad_link,
  pattern,
  glob,
  parse_int,
  io,
  json
} asar_status;

ASAR_API(asar_status) asar_list_package(const char* archive,
                                        char* buf,
                                        size_t* buf_len,
                                        const char** list,
                                        size_t* list_len);

ASAR_API(asar_status) asar_extract_all(const char* archive,
                                       const char* dest);

ASAR_API(asar_status) asar_create_package(const char* archive,
                                          const char* dest);

#endif  // ASAR_RS_H_

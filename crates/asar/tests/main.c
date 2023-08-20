#include <stdio.h>
#include <stdlib.h>
#include "../include/asar.h"

int main() {
  size_t buflen = 0, listlen = 0;
  const char* path = "./crates/asar/tests/expected/packthis.asar";
  asar_status r = asar_list_package(path, NULL, &buflen, NULL, &listlen);
  char* buf = (char*) malloc(buflen);
  const char** list = (const char**) malloc(listlen * sizeof(const char*));
  r = asar_list_package(path, buf, &buflen, list, &listlen);
  printf("buflen: %llu\n", buflen);
  printf("listlen: %llu\n", listlen);
  for (size_t i = 0; i < listlen; ++i) {
    printf("%s\n", *(list + i));
  }
  free(list);
  free(buf);

  r = asar_extract_all("./crates/asar/tests/input/extractthis.asar",
                       "./crates/asar/tmp/extractthis-c");

  r = asar_create_package("./crates/asar/tests/input/packthis",
                          "./crates/asar/tmp/packthis-c.asar");
  return 0;
}

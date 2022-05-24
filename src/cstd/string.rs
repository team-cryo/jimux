// pub extern "C" fn strlen(const str: &[char])
// {
// 	int i;
// 	for (i = 0; str[i] != '\0'; i++);
// 	return i;
// }

// void memcpy(void *dest, const void *src, int n) {
// 	int *tsrc = (int*)src;
// 	int *tdest = (int*)dest;

// 	for (int i = 0; i < n; i++) {
// 		tdest[i] = tsrc[i];
// 	}
// }

// void memset(void *ptr, int val, int n) {
// 	int *temp = (int*)ptr;

// 	for (int i = 0; i < n; i++) {
// 		temp[i] = val;
// 	}
// }

// char *strcpy(char *dest, const char *src) {
// 	for (int i = 0; src[i] != '\0'; i++) {
// 		dest[i] = src[i];
// 	}

// 	return dest;
// }

// char *strcat (char *dest, const char *src) {
// 	char *original = dest;

// 	while (*dest != 0) dest++;

// 	while (*src != 0) *dest++ = *src++;
// 	*dest = 0;

// 	return original;
// }

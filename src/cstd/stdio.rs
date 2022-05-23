// char* itoa(int value, char* result, int base) {
//     // check that the base if valid
//     if (base < 2 || base > 36) { *result = '\0'; return result; }

//     char* ptr = result, *ptr1 = result, tmp_char;
//     int tmp_value;

//     do {
//         tmp_value = value;
//         value /= base;
//         *ptr++ = "zyxwvutsrqponmlkjihgfedcba9876543210123456789abcdefghijklmnopqrstuvwxyz" [35 + (tmp_value - value * base)];
//     } while ( value );

//     // Apply negative sign
//     if (tmp_value < 0) *ptr++ = '-';
//     *ptr-- = '\0';
//     while(ptr1 < ptr) {
//         tmp_char = *ptr;
//         *ptr--= *ptr1;
//         *ptr1++ = tmp_char;
//     }
//     return result;
// }

// void sprintf(char *str, const char *format, ...) {

// }

// void printf(char* str, ...) {
// 	va_list args;

// 	va_start(args, str);

// 	for (int i = 0; str[i] != '\0'; i++) {
// 		if (str[i] == '\n') {
// 			io_newline();
// 			continue;
// 		}

// 		if (str[i] == '%' && str[i + 1] == 'd' || str[i + 1] == 'i') {
// 			int arg = va_arg(args, int);
//             char res[128];
//             itoa(arg, res, 10);
// 			io_print_string(res);
//             i++;
//             continue;
// 		}

//         if (str[i] == '%' && str[i + 1] == 'u') {
//             int arg = va_arg(args, unsigned int);
//             char res[128];
//             itoa(arg, res, 10);
//             io_print_string(res);
//             i++;
//             continue;
//         }

//         if (str[i] == '%' && str[i + 1] == 'o') {
//             int arg = va_arg(args, int);
//             char res[128];
//             itoa(arg, res, 8);
//             io_print_string(res);
//             i++;
//             continue;
//         }

//         if (str[i] == '%' && str[i + 1] == 'x' || str[i + 1] == 'X') {
//             int arg = va_arg(args, int);
//             char res[128];
//             itoa(arg, res, 16);
//             io_print_string(res);
//             i++;
//             continue;
//         }

//         if (str[i] == '%' && str[i + 1] == 'f' || str[i + 1] == 'F') {
//             int arg = va_arg(args, double);
//             char res[128];
//             itoa(arg, res, 16);
//             io_print_string(res);
//             i++;
//             continue;
//         }

//         if (str[i] == '%' && str[i + 1] == 'c') {
//             int arg = va_arg(args, int);
//             io_put_char(arg);
//             i++;
//             continue;
//         }

//         if (str[i] == '%' && str[i + 1] == 's') {
//             char *arg = va_arg(args, char*);
//             vga_print_string(arg);
//             i++;
//             continue;
//         }

// 		putchar(str[i]);
// 	}

// 	va_end(args);
// }

// void putchar(int c) {
// 	io_put_char(c);
// }

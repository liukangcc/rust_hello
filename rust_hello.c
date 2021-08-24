#include <rtthread.h>

extern void rust_hello(void);

int rust_hello_example(void)
{
    rust_hello();
    return 0;
}
MSH_CMD_EXPORT(rust_hello_example, rust language hell0 sample)

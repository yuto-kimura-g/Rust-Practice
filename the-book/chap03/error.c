#include <stdio.h>
int main(void) {
    int array[5] = {1, 2, 3, 4, 5};
    int idx = 128;
    int error_maker = array[idx];
    printf("value : %d\n", error_maker);
    return 0;
}

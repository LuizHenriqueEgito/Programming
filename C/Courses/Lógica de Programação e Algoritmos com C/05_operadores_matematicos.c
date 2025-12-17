#include <stdio.h>

int main() {
    int num1 = 10, num2 = 2;
    int add = num1 + num2;
    int sub = num1 - num2;
    int mul = num1 * num2;
    int div = num1 / num2;
    int mod = num1 % num2;

    printf("Add: %d + %d = %d\n", num1, num2, add);
    printf("Sub: %d - %d = %d\n", num1, num2, sub);
    printf("Mul: %d * %d = %d\n", num1, num2, mul);
    printf("Mul: %d / %d = %d\n", num1, num2, div);
    printf("Mod: %d %% %d = %d\n", num1, num2, mod);
    return 0;
}
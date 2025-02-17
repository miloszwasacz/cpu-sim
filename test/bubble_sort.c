#include "simpleio.h"

int main() {
    int arr[10] = {3,2,4,1,9,7,8,0,5,6};
    int n = 10;
    while (n > 1)
    {
        for(int j = 0; j < n - 1; j++)
        {
            if (arr[j] > arr[j + 1])
            {
                int t = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = t;
            }
        }
        n--;
    }

    putchar('[');
    putchar(arr[0] + '0');
    for (int i = 1; i < 10; i++) {
        fputs(", ", STDOUT);
        putchar(arr[i] + '0');
    }
    puts("]");
}
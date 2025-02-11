// #include<stdio.h>
#include "syscall.h"

int test(int a, char b) {
    int c = a + b;
    return c;
}

int main() {
    int arr[10] = {3,2,4,1,9,7,8,10,5,6};

    volatile int brr[4] = {1,2,3,4};
    
    int d = test(brr[1], brr[2]);
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
    exit(arr[0] + d);

    // printf("%d", arr[0]);
}
// #include<stdio.h>

int test(int a, char b) {
    int c = a + b;
    return c;
}

int main() {
    int arr[10] = {3,2,4,1,9,7,8,0,5,6};

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

    char chars[22];
    chars[0] = '['; chars[20] = ']'; chars[21] = '\n';
    for (int i = 0; i < 10; i++)
        chars[i * 2 + 1] = arr[i] + '0';
    for (int i = 2; i < 20; i += 2)
        chars[i] = ',';
        
    _write(1, chars, 22);
}
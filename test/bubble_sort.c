// #include<stdio.h>

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

    const int LEN = 23;
    char chars[LEN];
    chars[0] = '['; chars[LEN - 3] = ']'; chars[LEN - 2] = '\n'; chars[LEN - 1] = '\0';
    for (int i = 0; i < 10; i++)
        chars[i * 2 + 1] = arr[i] + '0';
    for (int i = 2; i < LEN - 3; i += 2)
        chars[i] = ',';
        
    _write(1, chars, LEN);
}
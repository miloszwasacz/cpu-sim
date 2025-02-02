// #include<stdio.h>

// int test(int a, char b) {
//     int c = a + b;
//     return c;
// }

int main() {
    int arr[10];
    arr[0] = 3;
    arr[1] = 2;
    arr[2] = 4;
    arr[3] = 1;
    arr[4] = 9;
    arr[5] = 7;
    arr[6] = 8;
    arr[7] = 10;
    arr[8] = 5;
    arr[9] = 6;

    int brr[4] = {1,2,3,4};
    
    // int d = test(2, 3);
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
    }
    // printf("%d", arr[0]);
    // std::cout << arr[0] << "\n";
}

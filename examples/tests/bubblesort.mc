void bubblesort(int * arr, int len) {
    for(int i = 0; i < len; i = i + 1) {
        for (int j = 0; j < len - 1 - i; j = j + 1) {
            if (arr[j] > arr[j + 1]) {
                int tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}

int main() {
    int arr[5];
    arr[0] = 2;
    arr[1] = 5;
    arr[2] = 1;
    arr[3] = 3;
    arr[4] = 4;
    
    bubblesort(arr, 5);

    for (int i = 0; i < 5; i = i + 1) {
        if (arr[i] != i + 1)
            return 1;
    }
    return 0;
}

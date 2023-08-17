int partition(int * arr, int from, int to) {
    int split = from;
    int pivot = arr[from];
    for (int i = from + 1; i < to; i = i + 1) {
        if (arr[i] < pivot) {
            split = split + 1;
            int tmp = arr[split];
            arr[split] = arr[i];
            arr[i] = tmp;
        }
    }

    int tmp = arr[split];
    arr[split] = arr[from];
    arr[from] = tmp;

    return split;
}

void quicksort(int * arr, int from, int to) {
    if (from < to) {
        int split = partition(arr, from, to);    
        quicksort(arr, from, split);
        quicksort(arr, split + 1, to);
    }
}

int main() {
    int arr[5];
    arr[0] = 1;
    arr[1] = 5;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 4;

    int x = 0;
    
    quicksort(arr, 0, 5);

    for (int i = 0; i < 5; i = i + 1) {
        if (arr[i] != i + 1)
            return i;
        x = x + arr[i];
    }
    arr[0] = x;
    return arr[0];
}

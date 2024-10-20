void exit(int retnum) {
    @(93, retnum);
}

int main() {
    char arr[6];
    arr[0] = 'h';

    @(64, 1, arr, 1);
    exit(123);
    return 0;
}

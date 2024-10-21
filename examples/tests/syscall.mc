void exit(int retnum) {
    @(93, retnum);
}

int main() {
    char arr[6];
    arr[0] = 'h';
    arr[1] = 'e';
    arr[2] = 'l';
    arr[3] = 'l';
    arr[4] = 'o';
    arr[5] = '\n';

    @(64, 1, arr, 6);
    exit(123);
    return 0;
}

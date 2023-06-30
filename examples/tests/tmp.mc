int main() {
    int i = 1;
    int j = 10;
    int * pi = &i;
    int x = i + j;
    i = 56;
    *pi = 123;
    return i;
}

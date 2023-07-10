int fib(int n) {
    int a = 0;
    int b = 1;
    for (int i = 0; i < n; i = i + 1) {
        int tmp = b;
        b = a + b;
        a = tmp;
    }
    return a;
}

int main() {
    return fib(40);
}

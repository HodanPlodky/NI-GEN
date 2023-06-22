int fib(int n) {
    if (n <= 0)
        return 0;
    else if (n <= 1)
        return 1;
    else
        return fib(n - 1) + fib(n - 2);
}

int betterfib(int n) {
    int a = 0;
    int b = 1;
    return a;
}

int main() {
    int i = 0;
    int sum = 0;
    while (i < 10) {
        sum = sum + fib(i + 10);
        i = i + 1;
    }
    return sum;
}

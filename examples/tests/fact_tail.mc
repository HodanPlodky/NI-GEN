int fact_inner(int n, int acc) {
    if (n <= 1) 
        return acc;
    return fact_inner(n - 1, acc * n);
}

int fact(int n) {
    return fact_inner(n, 1);
}

int main() {
    return fact(5);
}

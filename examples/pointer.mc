void f(int * p) {
    *p = *p + 1;
}

int main() {
    int a = 1;
    int * p;
    p = &a;
    *p = 10;
    f(&a);
    return a;
}

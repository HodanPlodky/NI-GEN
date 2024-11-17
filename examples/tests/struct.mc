struct A {
    int a;
    int b;
}

int add(A a) {
    return a.a + a.b;
}

A create() {
    struct A a;
    a.a = 1;
    a.b = 2;
    return a;
}

int main() {
    return add(create());
}

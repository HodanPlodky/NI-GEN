int main() {
    int * p;
    int * q;
    int x;
    int y;
    p = &x;
    q = &y;
    p = q;
    return *p;
}

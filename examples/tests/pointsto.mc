int main() {
    int * p;
    int * q;
    int x;
    int y;
    p = &x;
    p = q;
    q = &y;
    return *p;
}

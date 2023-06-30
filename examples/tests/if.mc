int main() {
    if (1) {
        return 1;
    }
    return 0;
}

int f() {
    int x = 1;
    if (1) {
        x = 2;
    }
    return x;
}

int g() {
    int x;
    if (1) {
        x = 5;
    }
    else {
        x = 10;
    }
    return x;
}

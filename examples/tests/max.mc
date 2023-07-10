int max(int a, int b) {
    if (a < b)
        return b;
    else 
        return a;
}

int main() {
    int x = 7;
    int y = 18;
    return max(max(10, y), max(2, x));
}

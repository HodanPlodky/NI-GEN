int main() {
    int sum = 0;
    int i = 0;
    while (i < 1000) {
        int j = 0;
        while (j < 1000) {
            sum = sum + i + j;
            j = j + 1;
        }
        i = i + 1;
    }
    return sum;
}

#include <iostream>
#include <vector>

void print_numbers() {
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    for (int num : numbers) {
        std::cout << "Number: " << num << std::endl;
    }
}

int main() {
    std::cout << "Hello from C++!" << std::endl;
    print_numbers();
    return 0;
} 
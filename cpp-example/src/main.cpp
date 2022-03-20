#include <iostream>
#include <string>

void printIntPointer(const std::string name, int* pointer);

/// This demonstrates that C++ still allows you to access invalid memory
int main() {
    const auto foo = new int(22);
    printIntPointer("foo", foo);

    *foo = 87;
    printIntPointer("foo", foo);

    // the following commented out lines produce a compile error
    // foo = new int(102);
    // printIntPointer("foo", foo);

    delete foo;
    printIntPointer("foo", foo);

    return 0;
}

void printIntPointer(const std::string name, const int* pointer) {
    std::cout << name << " (pointer): " << pointer  << std::endl;
    std::cout << name << " (value)  : " << *pointer << std::endl;
}

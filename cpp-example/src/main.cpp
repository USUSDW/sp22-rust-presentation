#include <iostream>
#include <string>

template <typename T>
struct NamedPointer {
    std::string name;
    T* pointer;

    NamedPointer(std::string name, T* pointer):
        name(name), pointer(pointer) {}
};

template <typename T>
void printPointer(const NamedPointer<T> namedPointer);

int main() {
    const auto foo = NamedPointer<int>(std::string("foo"), new int(22));
    printPointer(foo);

    *foo.pointer = 87;
    printPointer(foo);

    delete foo.pointer;
    printPointer(foo);

    return 0;
}

void intPointerDemo() {
    const auto foo = NamedPointer<int>(std::string("foo"), new int(22));
    printPointer(foo);

    *foo.pointer = 87;
    printPointer(foo);

    delete foo.pointer;
    printPointer(foo);
}

template <typename T>
void printPointer(const NamedPointer<T> namedPointer) {
    std::cout << namedPointer.name << " (pointer): " << namedPointer.pointer  << std::endl;
    std::cout << namedPointer.name << " (value)  : " << *namedPointer.pointer << std::endl;
}
